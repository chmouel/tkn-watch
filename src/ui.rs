use std::cmp::Ordering;

use askama::Template;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use kube::{api::ListParams, core::DynamicObject, Api};

use crate::{tekton::types::PipelineRun, utils};

#[derive(Template)]
#[template(path = "pipelinerun-status.html")]
pub struct PipelineRunStatusTemplate<'a> {
    pub name: &'a str,
    pub duration: &'a str,
    pub since_word: &'a str,
    pub tasks: &'a Vec<String>,
    pub first_asterisk: &'a String,
    pub refresh_seconds: &'a u64,
}

pub fn format_pr(pr: &PipelineRun, refresh_seconds: u64) -> String {
    if pr.status.is_none() || pr.status.clone().unwrap().start_time.is_none() {
        return format!(
            "PipelineRun {} is {} to start.",
            utils::colorit("cyan", &pr.metadata.name),
            utils::colorit("yellow", "pending")
        );
    }
    let status = pr.status.as_ref().unwrap();
    let start_time = status.start_time.as_ref().unwrap();
    let humantime = utils::parse_dt_as_duration(start_time);
    //  check if pipeline is running
    let mut since_word = "finished";
    let mut first_asterisk = utils::colorit("green", "✓");
    let is_running = status.completion_time.is_some();
    if !is_running {
        since_word = "started";
        first_asterisk = utils::colorit("yellow", "*");
    }
    if status.conditions[0].status == "False" {
        since_word = "failed";
        first_asterisk = utils::colorit("red", "✗");
    } else if status.conditions[0].status == "True" && !is_running {
        first_asterisk = utils::colorit("green", "✓");
    }
    let mut sorted = status
        .task_runs
        .iter()
        .map(|(_, status)| status)
        .collect::<Vec<_>>();

    sorted.sort_by(|a, b| {
        match a.status["status"]
            .start_time
            .cmp(&b.status["status"].start_time)
        {
            Ordering::Equal => a.pipeline_task_name.cmp(&b.pipeline_task_name).reverse(),
            _ => a.status["status"]
                .start_time
                .cmp(&b.status["status"].start_time),
        }
    });

    let tasks = sorted
        .iter()
        .map(|tstatus| {
            let status = &tstatus.status["status"];
            let cond = &status.conditions[0];
            let tstatus2 = cond.reason.as_str().to_lowercase();

            let start_char = utils::get_running_char(&tstatus2);
            let mut ret = format!("{} {:-20}\n", start_char, tstatus.pipeline_task_name);
            // go over all the taskruns
            if !status.conditions.is_empty() && status.conditions[0].status != "True" {
                if let Some(sstep) = &status.steps {
                    ret += &sstep
                        .iter()
                        .filter(|s| s.terminated.is_some() || s.running.is_some())
                        .map(|s| {
                            if let Some(terminated) = &s.terminated.clone() {
                                format!(
                                    "  {} {}\n",
                                    utils::get_running_char(&terminated.reason.to_lowercase()),
                                    &s.name,
                                )
                            } else {
                                format!("  {} {}\n", utils::colorit("yellow", "*"), &s.name)
                            }
                        })
                        .collect::<String>();
                }
            }
            ret.trim_end_matches('\n').to_string()
        })
        .collect::<Vec<String>>();

    let tmpl = PipelineRunStatusTemplate {
        name: &pr.metadata.name,
        duration: &humantime,
        since_word,
        tasks: &tasks,
        first_asterisk: &first_asterisk,
        refresh_seconds: &refresh_seconds,
    };
    tmpl.render().unwrap()
}

pub async fn refresh_pr(
    pr_name: &str,
    api: Api<DynamicObject>,
    refresh_seconds: u64,
) -> anyhow::Result<()> {
    loop {
        let pr = crate::tekton::pipelinerun::get(api.clone(), pr_name).await?;
        // move cursor to top left
        print!("\x1b[0;0H");
        print!("\x1b[J");

        println!("{}", format_pr(&pr, refresh_seconds));
        if let Some(status) = pr.status {
            if status.completion_time.is_some() {
                if status.conditions[0].status == "False" {
                    // return an error
                    println!();
                    return Err(anyhow::anyhow!("pipelinerun has failed"));
                }
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(refresh_seconds));
    }
    Ok(())
}

pub async fn select_pipelinerun(api: Api<DynamicObject>, last: bool) -> anyhow::Result<String> {
    // TODO: only runnning ones
    let prs = api.list(&ListParams::default()).await?;
    // collect all prs items name and separate by \n
    let mut names = prs.items.iter().
    // filter if running or not
    collect::<Vec<_>>();

    names.sort_by(|a, b| {
        a.metadata
            .creation_timestamp
            .cmp(&b.metadata.creation_timestamp)
            .reverse()
    });
    // get pipelinerun name
    let names = names
        .iter()
        .filter(|pr| pr.metadata.name.is_some())
        .map(|pr| pr.metadata.name.clone().unwrap())
        .collect::<Vec<_>>();

    if last {
        return Ok(names[0].to_string());
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a PipelineRun:")
        .default(0)
        .items(&names)
        .interact()
        .unwrap();
    Ok(names[selection].to_string())
}
