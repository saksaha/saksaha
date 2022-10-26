use crate::utils::Kommand;
use crate::CIError;
use std::env::Args;
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "yarn";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let sak_proof_wasm_path = PathBuf::from("source/sak_proof_wasm");
    sak_proof_wasm_path.try_exists()?;

    let args_1: Vec<String> = ["run", "dev"].iter().map(|s| s.to_string()).collect();

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .current_dir(sak_proof_wasm_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
