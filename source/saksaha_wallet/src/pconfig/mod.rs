use crate::{fs, WalletError};
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
        // let pconfig_fd =
        //     sak_fs::get_app_root_path(APP_NAME)?.join(PCONFIG_FILE_NAME);
        let pconfig_fd = get_pconfig_file_path()?;

        if !pconfig_fd.exists() {
            // std::fs::create_dir_all(pconfig_fd.clone())?;
        } else {
            return Err(format!("Credential has already been created").into());
        }

        // std::fs::write(receipt_path, serde_json::to_string_pretty(&receipt)?)?;

        // println!(
        //     "\nPConfig is successfully created under {}",
        //     self.acc_addr.yellow(),
        // );

        Ok(())
    }

    fn get_pconfig_path() -> Result<PathBuf, WalletError> {
        // let pconfig_fd =
        //     sak_fs::get_app_root_path(APP_NAME)?.join(PCONFIG_FILE_NAME);
        let pconfig_fd = get_pconfig_file_path()?;

        Ok(pconfig_fd)
    }
}

fn get_pconfig_file_path() -> Result<PathBuf, WalletError> {
    let p = fs::config_dir()?.join(PCONFIG_FILE_NAME);
    Ok(p)
}
