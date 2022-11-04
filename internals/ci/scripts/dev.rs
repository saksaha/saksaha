use crate::logln;
use crate::tasks;
use crate::utils::Kommand;
use crate::vec_of_strings;
use crate::CIError;
use std::env::Args;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    if !tasks::naively_check_if_prebuild_has_done()? {
        logln!("prebuild has not been done yet. Will do");

        tasks::build_system_contracts()?;
        tasks::build_3rd_party_contracts()?;
        tasks::build_circuit_params()?;
    }

    let program = "cargo";

    let cli_args = args.collect();

    let args_1 = vec_of_strings!["run", "--package", "saksaha_network", "--"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
