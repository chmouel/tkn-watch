use std::{cmp::Ordering, vec};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use kube::{core::DynamicObject, Api};

use crate::{
    tekton::{pipelinerun, types::PipelineRun},
    utils,
};

pub fn format_pr(pr: &PipelineRun) -> String {
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
    let mut sorted = status.task_runs.values().collect::<Vec<_>>();

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

    let pac = vec!["event-type", "url-org", "url-repository", "sha"];
    let mut ret = String::new();

    if pr.metadata.labels.is_some() {
        let metadata = pr.metadata.labels.clone().unwrap();
        let pac_title = metadata
            .iter()
            .map(|(k, v)| (k.split('/').last().unwrap(), v.as_str()))
            .filter(|(k, _v)| pac.contains(k))
            .collect::<std::collections::HashMap<&str, &str>>();
        if pac_title.len() == 4 {
            ret = format!(
                "{} {} on {}/{} (sha {})\n",
                utils::colorit("magenta", "!"),
                pac_title.get("event-type").unwrap_or(&"--"),
                utils::colorit("bold", pac_title.get("url-org").unwrap_or(&"--")),
                utils::colorit("bold", pac_title.get("url-repository").unwrap_or(&"--")),
                utils::colorit("italic", pac_title.get("sha").unwrap_or(&"--")),
            );
        }
    }

    ret.push_str(
        format!(
            "{} {} - {} {}\n\nTASKS:\n\n",
            first_asterisk, &pr.metadata.name, since_word, humantime
        )
        .as_str(),
    );
    for task in tasks {
        ret.push_str(format!("{}\n", task).as_str());
    }

    ret
}

pub async fn refresh_pr(
    pr_name: &str,
    api: Api<DynamicObject>,
    refresh_seconds: u64,
    quiet: bool,
) -> anyhow::Result<()> {
    let sp = if quiet {
        spinner::SpinnerBuilder::new(String::new())
            .spinner(vec![])
            .start()
    } else {
        spinner::SpinnerBuilder::new(format!(
            "Refreshing pipelinerun status every {} seconds. Press Ctrl+C to quit.",
            refresh_seconds
        ))
        .start()
    };
    loop {
        let pr = crate::tekton::pipelinerun::get(api.clone(), pr_name).await?;

        if !quiet {
            // move cursor to top left
            print!("\x1b[0;0H");
            print!("\x1b[J");
            println!("{}", format_pr(&pr));
        }

        if let Some(status) = pr.status {
            if status.completion_time.is_some() {
                if status.conditions[0].status == "False" {
                    // return an error
                    if quiet {
                        // return a non-zero exit code
                        std::process::exit(1);
                    }
                    sp.message(utils::get_running_char("failed"));
                    return Err(anyhow::anyhow!("pipelinerun has failed"));
                }
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(refresh_seconds));
    }
    sp.message(utils::get_running_char("succeeded"));
    Ok(())
}

pub async fn select_pipelinerun(
    api: Api<DynamicObject>,
    last: bool,
    quiet: bool,
) -> anyhow::Result<String> {
    let prs = pipelinerun::running(api.clone()).await?;

    if prs.is_empty() {
        return Err(anyhow::anyhow!("no running pipelinerun has been found"));
    }

    if prs.len() == 1 {
        return Ok(prs[0].to_string());
    }

    if last {
        return Ok(prs[0].to_string());
    }

    if quiet {
        return Err(anyhow::anyhow!("you need to specify the pipelinerun name"));
    }

    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a PipelineRun:")
        .default(0)
        .items(&prs)
        .interact()
        .unwrap();
    Ok(prs[selection].to_string())
}
