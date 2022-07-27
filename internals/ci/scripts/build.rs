use crate::{log, CIError};
use clap::ArgMatches;
use colored::Colorize;
use std::env::Args;
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Build;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "cargo";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let args_1: Vec<String> = ["build", "--package", "saksaha_network", "--"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let args = [args_1, cli_args].concat();

    log!(
        "Found subcommand, script: {}, executing `{} {}`",
        "dev",
        program.yellow(),
        args.join(" ").yellow(),
    );

    Cmd::new(program)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
