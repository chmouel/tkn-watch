use std::fs;

use clap_complete::{generate_to, Shell};
use Shell::*;

include!("src/cli.rs");

fn main() {
    let outdir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("misc/")
        .join("completions/");
    fs::create_dir_all(&outdir).expect("cannot create directory");

    let mut cmd = command();
    for shell in [Bash, Zsh, PowerShell, Fish, Elvish] {
        generate_to(shell, &mut cmd, "tkn-watch", &outdir).expect("cannot generate completions");
    }
}
