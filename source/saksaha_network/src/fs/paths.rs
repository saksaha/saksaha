use super::SaksahaFSError;
use std::path::PathBuf;

const APP_NAME: &str = "saksaha";

pub fn config_dir() -> Result<PathBuf, SaksahaFSError> {
    sak_dir::get_config_dir(APP_NAME)
}

pub fn acc_dir(public_key: &String) -> Result<PathBuf, SaksahaFSError> {
    let p = sak_dir::get_config_dir(APP_NAME)?.join(public_key);
    Ok(p)
}
