use clap::Command;

pub fn command() -> clap::Command<'static> {
    let cmd = Command::new("tkn-watch")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(
            clap::Arg::new("namespace")
                .short('n')
                .long("namespace")
                .value_name("NAMESPACE")
                .help("The namespace scope for this CLI request")
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("last")
                .short('l')
                .long("last")
                .help("Choose the last pipelinerun"),
        )
        .arg(
            clap::Arg::new("pipelinerun")
                .value_name("PIPELINERUN")
                .help(
                    "The name of the pipelinerun to watch or you can use the interactive selector \
                 if you don't specify one",
                )
                .takes_value(true),
        )
        .arg(
            clap::Arg::new("refresh-seconds")
                .value_name("REFRESH_SECONDS")
                .help("The number of seconds to wait between refreshes")
                .takes_value(true)
                .default_value("3")
                .short('r')
                .long("refresh-seconds"),
        )
        .arg(
            clap::Arg::new("file")
                .short('f')
                .long("file")
                .value_name("FILE")
                .hide(true)
                .help("The json file to read the PipelineRun from instead of running one")
                .takes_value(true),
        );
    cmd
}
