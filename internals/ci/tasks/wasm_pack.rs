use crate::{log, CIError};
use std::{
    env,
    path::{Path, PathBuf},
};

pub(crate) fn check_wasm_pack() -> Result<(), CIError> {
    log!("check wasm pack");

    find_command("wasm-pack").ok_or(format!(
        "wasm-pack is not found. You may have to install it.\n\
        Please refer to 'https://rustwasm.github.io/wasm-pack/installer/'\n\
        or execute curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh",
    ))?;

    Ok(())
}

fn find_command<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).find_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        })
    })
}
