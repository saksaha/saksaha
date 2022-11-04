use crate::utils::Kommand;
use crate::{logln, tasks};
use crate::{vec_of_strings, CIError};
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

    let wasm_pack_needs_nightly_so_excluded =
        vec_of_strings!["--workspace", "--exclude", "sak_proof_wasm"];

    let cargo_test_args =
        vec_of_strings!["--", "--nocapture", "--show-output", "--test-threads", "1"];

    let args = vec![
        vec_of_strings!["test"],
        cli_args,
        wasm_pack_needs_nightly_so_excluded,
        cargo_test_args,
    ]
    .concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to run");

    Ok(())
}
