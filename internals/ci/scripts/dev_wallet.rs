use crate::utils::Kommand;
use crate::{vec_of_strings, CIError};
use std::env::Args;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "cargo";

    let cli_args = args.collect();

    let args_1 = vec_of_strings!["run", "--package", "saksaha_wallet", "--"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
