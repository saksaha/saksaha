use crate::utils::Kommand;
use crate::{logln, CIError};
use crate::{tasks, vec_of_strings};
use std::env::Args;
use std::path::PathBuf;
use std::process::Stdio;

const WASM_PACK_VERSION: &str = "wasm-pack 0.10.2";

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    tasks::check_wasm_pack()?;

    let sak_proof_wasm_path = PathBuf::from("source/sak_proof_wasm");
    sak_proof_wasm_path.try_exists()?;

    let program = "wasm-pack";

    let wasm_pack_version = Kommand::new(program, vec_of_strings!["--version"], None)?
        .current_dir(&sak_proof_wasm_path)
        .output()
        .expect("failed to run");

    let wasm_pack_version = String::from_utf8(wasm_pack_version.stdout)?;

    if wasm_pack_version.trim() != WASM_PACK_VERSION {
        logln!(
            "Your local wasm-pack is diffrent version than the one we tested, \
            tested: {:?}, yours: {:?}",
            WASM_PACK_VERSION,
            wasm_pack_version,
        );
    }

    let program = "rustup";

    let cli_args = args.collect();

    let args_1 = vec_of_strings!["run", "nightly", "wasm-pack", "build", "--target", "web"];

    let args = [args_1, cli_args].concat();

    Kommand::new(program, args, None)?
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(sak_proof_wasm_path)
        .output()
        .expect("failed to run");

    Ok(())
}
