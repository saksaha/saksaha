use crate::{
    logln,
    paths::Paths,
    utils::{self, Kommand},
    vec_of_strings, CIError,
};
use std::{env::Args, process::Stdio};

const WASM_PACK_VERSION: &str = "wasm-pack 0.10.2";

pub(crate) fn check_wasm_pack() -> Result<(), CIError> {
    logln!("check wasm pack");

    utils::find_command("wasm-pack").ok_or(format!(
        "wasm-pack is not found. You may have to install it.\n\
        Please refer to 'https://rustwasm.github.io/wasm-pack/installer/'\n\
        or execute curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh",
    ))?;

    Ok(())
}

pub(crate) fn build_sak_proof_wasm(args: Args) -> Result<(), CIError> {
    // let source_path = Paths::source()?;
    // let sak_proof_wasm_path = source_path.join("sak_proof_wasm");

    // let program = "wasm-pack";

    // let wasm_pack_version = Kommand::new(program, vec_of_strings!["--version"], None)?
    //     .current_dir(&sak_proof_wasm_path)
    //     .output()
    //     .expect("failed to run");

    // let wasm_pack_version = String::from_utf8(wasm_pack_version.stdout)?;

    // if wasm_pack_version.trim() != WASM_PACK_VERSION {
    //     logln!(
    //         "Your local wasm-pack is diffrent version than the one we tested, \
    //         tested: {:?}, yours: {:?}",
    //         WASM_PACK_VERSION,
    //         wasm_pack_version,
    //     );
    // }

    // let program = "rustup";

    // let cli_args = args.collect();

    // let args_1 = vec_of_strings!["run", "nightly", "wasm-pack", "build", "--target", "web"];

    // let args = [args_1, cli_args].concat();

    // Kommand::new(program, args, None)?
    //     .stdout(Stdio::inherit())
    //     .stderr(Stdio::inherit())
    //     .current_dir(sak_proof_wasm_path)
    //     .output()
    //     .expect("failed to run");

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
