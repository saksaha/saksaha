use crate::log;
use crate::tasks;
use crate::utils::Kommand;
use crate::CIError;
use colored::Colorize;
use std::env::Args;
use std::process::{Command as Cmd, Stdio};

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    tasks::clean_prebuild()?;
    tasks::build_system_contracts()?;
    tasks::build_3rd_party_contracts()?;

    let program = "cargo";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let args_1: Vec<String> = ["run", "--package", "saksaha_network", "--"]
        .iter()
        .map(|s| s.to_string())
        .collect();

    let args = [args_1, cli_args].concat();

    // Kommand::new(program, args, None)?
    //     .stdout(Stdio::inherit())
    //     .stderr(Stdio::inherit())
    //     .output()
    //     .expect("failed to run");

    Ok(())
}
