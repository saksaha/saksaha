use crate::utils::Kommand;
use crate::{vec_of_strings, CIError};
use std::env::Args;
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    let program = "yarn";

    let cli_args = args.collect();

    let saksaha_wallet_web_path = PathBuf::from("source/saksaha_wallet_web");
    saksaha_wallet_web_path.try_exists()?;

    let args_1 = vec_of_strings!["dev"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .current_dir(saksaha_wallet_web_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
