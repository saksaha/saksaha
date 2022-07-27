use crate::log;
use crate::CIError;
use clap::ArgMatches;
use colored::Colorize;
use std::env::{self, Args};
use std::path::PathBuf;
use std::process::{Command as Cmd, Stdio};

pub(crate) struct Test;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "cargo";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let cargo_test_args: Vec<String> = ["--", "--nocapture", "--show-output"]
        .iter()
        .map(|a| a.to_string())
        .collect();

    let args =
        vec![vec!["test".to_string()], cli_args, cargo_test_args].concat();

    log!(
        "Found subcommand, script: {}, executing `{} {}`",
        "dev",
        program.yellow(),
        args.join(" ").yellow(),
    );

    // env::set_var("RUST_BACKTRACE", "1");

    Cmd::new(program)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
