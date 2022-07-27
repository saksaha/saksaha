use crate::{log, utils::Kommand, CIError};
use std::process::{Command as Cmd, Stdio};

pub(crate) fn build_3rd_party_contracts() {
    log!("build 3rd party contracts");
}

pub(crate) fn build_system_contracts() -> Result<(), CIError> {
    log!("build system contracts");

    let curr_dir = std::env::current_dir()?;

    let contract_paths = vec![curr_dir.join("source/sak_validator")];

    for p in contract_paths {
        if p.exists() {
            let program = "cargo";

            let args = ["wasm"].iter().map(|s| s.to_string()).collect();

            println!("current: {:?}", p);
            Kommand::new(program, args).current_dir(p).output()?;
        } else {
            return Err(format!("contract path should exist").into());
        }
    }

    // .current_dir();

    Ok(())
}
