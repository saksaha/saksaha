use crate::utils::Kommand;
use crate::CIError;
use crate::{tasks, vec_of_strings};
use std::env::Args;
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    tasks::check_wasm_pack()?;

    let sak_proof_wasm_path = PathBuf::from("source/sak_proof_wasm");
    sak_proof_wasm_path.try_exists()?;

    let program = "wasm-pack";

    let cli_args = args.collect();

    let args_1 = vec_of_strings!["build"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(sak_proof_wasm_path)
        .output()
        .expect("failed to run");

    Ok(())
}
