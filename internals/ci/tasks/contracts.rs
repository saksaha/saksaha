use crate::{log, paths::Paths, utils::Kommand, CIError};
use sak_contract_std::symbols;
use serde::{Deserialize, Serialize};
use std::{
    path::PathBuf,
    process::{Command as Cmd, Stdio},
    time::SystemTime,
};

#[derive(Serialize, Deserialize)]
struct ContractCompileReceipt {
    build_time: SystemTime,
    outputs: Vec<PathBuf>,
}

struct Contract {
    name: &'static str,
    path: PathBuf,
}

pub(crate) fn naively_check_if_prebuild_has_done() -> Result<bool, CIError> {
    let prebuild_path = Paths::prebuild()?;

    for file in std::fs::read_dir(prebuild_path)? {
        let f = file?;
        let file_name = f.file_name();

        if file_name == ".gitkeep" {
            // do nothing
        } else {
            return Ok(true);
        }
    }

    Ok(false)
}

pub(crate) fn build_3rd_party_contracts() -> Result<(), CIError> {
    log!("build 3rd party contracts");

    let externals_path = Paths::externals()?;

    let contracts = vec![Contract {
        name: "envelope_contract",
        path: externals_path.join("envelope_contract"),
    }];

    let receipt = contracts
        .into_iter()
        .map(|ctr| {
            let wasm_path = build_contract(ctr)?;
            let output_path = post_process_wasm(wasm_path)?;

            Ok(output_path)
        })
        .collect::<Result<Vec<PathBuf>, CIError>>()?;

    persist_build_receipt_file("external_contracts.build.json", receipt)?;

    Ok(())
}

pub(crate) fn build_system_contracts() -> Result<(), CIError> {
    log!("build system contracts");

    let source_path = Paths::source()?;

    let sys_contracts = vec![
        Contract {
            name: "sak_validator_contract",
            path: source_path.join("sak_validator_contract"),
        },
        Contract {
            name: "sak_mrs_contract",
            path: source_path.join("sak_mrs_contract"),
        },
    ];

    let receipt = sys_contracts
        .into_iter()
        .map(|ctr| {
            let wasm_path = build_contract(ctr)?;
            let output_path = post_process_wasm(wasm_path)?;

            Ok(output_path)
        })
        .collect::<Result<Vec<PathBuf>, CIError>>()?;

    persist_build_receipt_file("system_contracts.build.json", receipt)?;

    Ok(())
}

fn persist_build_receipt_file(file_name: &str, receipt: Vec<PathBuf>) -> Result<(), CIError> {
    let build_time = SystemTime::now();

    let receipt_path = Paths::prebuild()?.join(file_name);

    let receipt = ContractCompileReceipt {
        build_time,
        outputs: receipt,
    };
    let receipt_str = serde_json::to_string_pretty(&receipt)?;

    std::fs::write(receipt_path, receipt_str)?;

    Ok(())
}

// As of August 2022, we cannot have package-specific profiles.
// However, specifying 'lto' or 'opt_level' in workspace Cargo.toml causes
// errors so this is a workaround.
// https://stackoverflow.com/a/72085741
fn add_cargo_optimizing_flags(cmd: &mut Cmd) {
    cmd.env("CARGO_PROFILE_RELEASE_LTO", "true");
    cmd.env("CARGO_PROFILE_RELEASE_OPT_LEVEL", "z");
    cmd.env("CARGO_PROFILE_RELEASE_PANIC", "abort");
}

fn build_contract(ctr: Contract) -> Result<PathBuf, CIError> {
    if ctr.path.exists() {
        let curr_path = Paths::curr()?;

        let program = "cargo";

        let args = ["wasm"].iter().map(|s| s.to_string()).collect();

        log!(
            "building system contract, name: {}, path: {:?}",
            ctr.name,
            ctr.path,
        );

        let mut cargo = Kommand::new(program, args, Some(ctr.path))?;

        add_cargo_optimizing_flags(&mut cargo);

        cargo
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .output()
            .expect("failed to run");

        let wasm_path = curr_path
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{}.wasm", ctr.name));

        if !wasm_path.exists() {
            return Err(format!("compiled wasm does not exist").into());
        }

        let wasm_dest_path = Paths::prebuild()?.join(format!("{}.wasm", ctr.name));

        std::fs::copy(&wasm_path, &wasm_dest_path)?;

        Ok(wasm_dest_path)
    } else {
        return Err(format!("contract path should exist").into());
    }
}

fn post_process_wasm(wasm_path: PathBuf) -> Result<PathBuf, CIError> {
    let multi_return_symbols = vec![
        format!("{} i32 i32", symbols::INIT_FN),
        format!("{} i32 i32", symbols::QUERY_FN),
        format!("{} i32 i32 i32 i32", symbols::EXECUTE_FN),
    ];

    let ret =
        wasm_postprocess::make_wasm_have_multiple_returns(wasm_path, None, multi_return_symbols)?;

    Ok(ret)
}
