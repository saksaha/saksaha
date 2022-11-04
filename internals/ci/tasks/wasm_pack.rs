use crate::{logln, utils, CIError};

pub(crate) fn check_wasm_pack() -> Result<(), CIError> {
    logln!("check wasm pack");

    utils::find_command("wasm-pack").ok_or(format!(
        "wasm-pack is not found. You may have to install it.\n\
        Please refer to 'https://rustwasm.github.io/wasm-pack/installer/'\n\
        or execute curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh",
    ))?;

    Ok(())
}
