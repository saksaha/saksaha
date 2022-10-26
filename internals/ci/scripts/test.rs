use crate::utils::Kommand;
use crate::CIError;
use crate::{log, tasks};
use std::env::Args;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    if !tasks::naively_check_if_prebuild_has_done()? {
        log!("prebuild has not been done yet. Will do");

        tasks::build_system_contracts()?;
        tasks::build_3rd_party_contracts()?;
        tasks::build_circuit_params()?;
    }

    let program = "cargo";

    let cli_args: Vec<String> = args.map(|a| a.to_string()).collect();

    let cargo_test_args: Vec<String> =
        ["--", "--nocapture", "--show-output", "--test-threads", "1"]
            .iter()
            .map(|a| a.to_string())
            .collect();

    let args = vec![vec!["test".to_string()], cli_args, cargo_test_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
