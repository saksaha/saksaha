use crate::utils::Kommand;
use crate::{logln, CIError};
use crate::{tasks, vec_of_strings};
use std::env::Args;
use std::path::PathBuf;
use std::process::Stdio;

pub(crate) fn run(args: Args) -> Result<(), CIError> {
    tasks::check_yarn()?;

    tasks::build_sak_proof_wasm(args)?;

    tasks::copy_sak_proof_wasm()?;

    Ok(())
}
