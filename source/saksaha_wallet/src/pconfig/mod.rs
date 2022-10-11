use crate::{
    fs::{self, SaksahaWalletFS},
    WalletError,
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const PCONFIG_FILE_NAME: &'static str = "ACCOUNTS";

#[derive(Debug, Serialize, Deserialize)]
pub struct PConfig {
    public_key: String,
    secret: String,
}

impl PConfig {
    pub fn load() -> Result<PConfig, WalletError> {
        let pconfig_path = Self::get_pconfig_path()?;

        if pconfig_path.exists() {
        } else {
            return Err(format!("Persisted config does not exist").into());
        }

        let c = PConfig {
            public_key: "dummy".to_string(),
            secret: "dummy".to_string(),
        };

        Ok(c)
    }

    pub fn persist() -> Result<(), WalletError> {
        let pconfig_fd = get_pconfig_file_path()?;

        if !pconfig_fd.exists() {
        } else {
            return Err(format!("Credential has already been created").into());
        }

        Ok(())
    }

    fn get_pconfig_path() -> Result<PathBuf, WalletError> {
        let pconfig_fd = get_pconfig_file_path()?;

        Ok(pconfig_fd)
    }
}

fn get_pconfig_file_path() -> Result<PathBuf, WalletError> {
    let p = SaksahaWalletFS::config_dir()?.join(PCONFIG_FILE_NAME);
    Ok(p)
}
