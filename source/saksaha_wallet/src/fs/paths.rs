use crate::WalletError;
use std::path::PathBuf;

pub const APP_NAME: &'static str = "saksaha-wallet";

pub(crate) fn config_dir() -> Result<PathBuf, WalletError> {
    sak_fs::get_config_dir(APP_NAME)
}

pub(crate) fn acc_dir(public_key: &String) -> Result<PathBuf, WalletError> {
    let p = config_dir()?.join(public_key);
    Ok(p)
}
