use super::SaksahaFSError;
use std::path::PathBuf;

const APP_NAME: &str = "saksaha";

pub fn config_dir() -> Result<PathBuf, SaksahaFSError> {
    sak_fs::get_config_dir(APP_NAME)
}

pub fn acc_dir(public_key: &String) -> Result<PathBuf, SaksahaFSError> {
    let p = sak_fs::get_config_dir(APP_NAME)?.join(public_key);
    Ok(p)
}
