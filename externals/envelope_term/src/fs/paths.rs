use crate::EnvelopeError;
use std::path::PathBuf;

const APP_NAME: &str = "envelope-term";

pub fn config_dir() -> Result<PathBuf, EnvelopeError> {
    sak_dir::get_config_dir(APP_NAME)
}

pub fn acc_dir(acc_addr: &String) -> Result<PathBuf, EnvelopeError> {
    let p = config_dir()?.join(acc_addr);

    Ok(p)
}
