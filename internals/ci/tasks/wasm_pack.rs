use crate::{
    logln,
    paths::Paths,
    utils::{self, Kommand},
    vec_of_strings, CIError,
};
use std::{env::Args, process::Stdio};

const WASM_PACK_VERSION: &str = "wasm-pack 0.10.2";

#[allow(dead_code)]
pub(crate) fn check_wasm_pack() -> Result<(), CIError> {
    logln!("check wasm pack");

    utils::find_command("wasm-pack").ok_or(format!(
        "wasm-pack is not found. You may have to install it.\n\
        Please refer to 'https://rustwasm.github.io/wasm-pack/installer/'\n\
        or execute curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh",
    ))?;

    Ok(())
}

pub(crate) fn check_yarn() -> Result<(), CIError> {
    logln!("check yarn");

    utils::find_command("wasm-pack").ok_or(format!(
        "yarn is not found. You may have to install it.\n\
        Please refer to 'https://classic.yarnpkg.com'",
    ))?;

    Ok(())
}

pub(crate) fn build_sak_proof_wasm(args: Args) -> Result<(), CIError> {
    let source_path = Paths::source()?;
    let sak_proof_wasm_path = source_path.join("sak_proof_wasm");

    let program = "yarn";

    let cli_args = args.collect();

    let args_1 = vec_of_strings!["run", "build"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(sak_proof_wasm_path)
        .output()
        .expect("failed to run");

    Ok(())
}

pub(crate) fn copy_sak_proof_wasm() -> Result<(), CIError> {
    let source_path = Paths::source()?;
    let sak_proof_wasm_pkg_path = source_path.join("sak_proof_wasm/pkg");
    sak_proof_wasm_pkg_path.try_exists()?;

    let prebuild_path = Paths::prebuild()?;
    let prebuild_sak_proof_wasm_path = prebuild_path.join("sak_proof_wasm");

    logln!("Copying sak_proof_wasm compiled to prebuild path");

    utils::copy_dir_all(sak_proof_wasm_pkg_path, prebuild_sak_proof_wasm_path)?;

    Ok(())
}
