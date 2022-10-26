use crate::tasks;
use crate::utils::Kommand;
use crate::{log, CIError};
use colored::Colorize;
use std::env::Args;
use std::path::PathBuf;
use std::process::{Command as Cmd, Stdio};

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    tasks::check_wasm_pack()?;

    let sak_proof_wasm_path = PathBuf::from("source/sak_proof_wasm");

    sak_proof_wasm_path.try_exists()?;

    let program = "wasm-pack";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let args_1: Vec<String> = ["build"].iter().map(|s| s.to_string()).collect();

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(sak_proof_wasm_path)
        .output()
        .expect("failed to run");

    Ok(())
}
