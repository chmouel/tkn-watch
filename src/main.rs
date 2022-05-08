mod cli;
mod kubernetes;
mod tekton;
mod ui;
mod utils;

use anyhow::Ok;
use ui::select_pipelinerun;

use crate::{kubernetes::client as kclient, tekton::pipelinerun::from_json, ui::format_pr};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // check if we have an argument
    let config = cli::command().get_matches_from(std::env::args_os());
    let quiet = config.is_present("quiet");

    let refresh_seconds = config
        .value_of("refresh-seconds")
        .unwrap()
        .parse::<u64>()
        .unwrap();

    if let Some(jsonfile) = config.value_of("file") {
        let pr = from_json(jsonfile.to_string())?;
        println!("{}", format_pr(&pr, refresh_seconds));
        return Ok(());
    }
    let api = kclient(config.value_of("namespace")).await?;
    let pr_name = if let Some(pr) = config.value_of("pipelinerun") {
        pr.to_string()
    } else {
        select_pipelinerun(api.clone(), config.is_present("last"), quiet).await?
    };

    ui::refresh_pr(&pr_name, api, refresh_seconds, quiet).await?;
    Ok(())
}
