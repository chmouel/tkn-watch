use std::{iter::zip, vec};

use dialoguer::{theme::ColorfulTheme, FuzzySelect};
use kube::{core::DynamicObject, Api};

use crate::{
    tekton::{
        pipelinerun,
        types::{ChildReference, PipelineRun, TaskRunStatus},
    },
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

    if pr.child_status.is_none() || status.child_references.is_none() || status.conditions.is_none()
    {
        return format!(
            "PipelineRun {} is {} to start for waiting tasks.",
            utils::colorit("cyan", &pr.metadata.name),
            utils::colorit("yellow", "pending")
        );
    }

    let child_status = pr.child_status.as_ref().unwrap();
    if child_status.is_empty() || child_status[0].conditions.is_none() {
        return format!(
            "PipelineRun {} is {} to start for waiting task conditions.",
            utils::colorit("cyan", &pr.metadata.name),
            utils::colorit("yellow", "pending")
        );
    }

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
    let conditions = status.conditions.as_ref().unwrap();
    if conditions[0].status == "False" {
        since_word = "failed";
        first_asterisk = utils::colorit("red", "✗");
    } else if conditions[0].status == "True" && !is_running {
        first_asterisk = utils::colorit("green", "✓");
    }

    let tasks = zip::<&Vec<TaskRunStatus>, &Vec<ChildReference>>(
        child_status,
        status.child_references.as_ref().unwrap(),
    )
    .map(|(tstatus, reference)| {
        let cond = &tstatus.conditions.as_ref().unwrap()[0];
        let tstatus2 = cond.reason.as_str().to_lowercase();

        let start_char = utils::get_running_char(&tstatus2);
        let mut ret = format!("{} {:-20}\n", start_char, reference.pipeline_task_name);
        // go over all the taskruns
        if !conditions.is_empty() && conditions[0].status != "True" {
            if let Some(sstep) = &tstatus.steps {
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

    let pac = ["event-type", "url-org", "url-repository", "sha"];
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
        ret.push_str(format!("{task}\n").as_str());
    }

    ret
}

pub async fn refresh_pr(
    pr_name: &str,
    pipelinerun: Api<DynamicObject>,
    taskrun: Api<DynamicObject>,
    refresh_seconds: u64,
    quiet: bool,
) -> anyhow::Result<()> {
    let sp = if quiet {
        spinner::SpinnerBuilder::new(String::new())
            .spinner(vec![])
            .start()
    } else {
        spinner::SpinnerBuilder::new(format!(
            "Refreshing pipelinerun status every {refresh_seconds} seconds. Press Ctrl+C to quit."
        ))
        .start()
    };
    loop {
        let pr =
            crate::tekton::pipelinerun::get(pipelinerun.clone(), taskrun.clone(), pr_name).await?;

        if !quiet {
            // move cursor to top left
            print!("\x1b[0;0H");
            print!("\x1b[J");
            println!("{}", format_pr(&pr));
        }

        if let Some(status) = pr.status {
            if status.completion_time.is_some() && status.conditions.is_some() {
                if status.conditions.unwrap()[0].status == "False" {
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
