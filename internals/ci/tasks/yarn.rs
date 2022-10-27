use crate::{log, utils, CIError};

pub(crate) fn check_yarn() -> Result<(), CIError> {
    log!("check yarn");

    utils::find_command("wasm-pack")
        .ok_or(format!("yarn is not found. You may have to install it."))?;

    Ok(())
}
