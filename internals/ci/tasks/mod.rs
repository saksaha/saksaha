use crate::{log, utils::Kommand, CIError};
use std::{
    path::PathBuf,
    process::{Command as Cmd, Stdio},
};

struct Contract {
    name: &'static str,
    path: PathBuf,
}

pub(crate) fn build_3rd_party_contracts() {
    log!("build 3rd party contracts");
}

pub(crate) fn build_system_contracts() -> Result<(), CIError> {
    log!("build system contracts");

    let curr_dir = std::env::current_dir()?;

    let prebuild_path = curr_dir.join("source/prebuild");
    if !prebuild_path.exists() {
        return Err(format!("prebuild path does not exist").into());
    }

    let sys_contracts = vec![Contract {
        name: "sak_validator",
        path: curr_dir.join("source/sak_validator"),
    }];

    for ctr in sys_contracts {
        let wasm_path = build_contract(ctr)?;
        post_process_wasm(wasm_path)?;
    }

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
        let curr_dir = std::env::current_dir()?;

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

        let wasm_path = curr_dir
            .join("target/wasm32-unknown-unknown/release")
            .join(format!("{}.wasm", ctr.name));

        println!("power: {:?}", wasm_path);

        if !wasm_path.exists() {
            return Err(format!("compiled wasm does not exist").into());
        }

        let wasm_dest_path = curr_dir
            .join("source/prebuild")
            .join(format!("{}.wasm", ctr.name));

        std::fs::copy(&wasm_path, &wasm_dest_path)?;

        Ok(wasm_dest_path)
    } else {
        return Err(format!("contract path should exist").into());
    }
}

fn post_process_wasm(wasm_path: PathBuf) -> Result<(), CIError> {
    let output_path = {
        let mut p = wasm_path.clone();
        p.set_file_name("power");
        p
    };

    wasm_postprocess::make_wasm_have_multiple_returns(wasm_path, output_path);

    Ok(())
}
