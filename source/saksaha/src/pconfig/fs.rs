use crate::pconfig::PConfig;
use colored::Colorize;
use log::info;
use sak_crypto::ToEncodedPoint;
use sak_fs::FS;
use sak_logger::tinfo;
use sak_p2p_addr::UnknownAddr;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";

pub fn persist(
    pconfig: PConfig,
    target_path: PathBuf,
) -> Result<PConfig, String> {
    let serialized = match serde_yaml::to_string(&pconfig) {
        Ok(s) => s,
        Err(err) => {
            return Err(format!("Error serializing pconfig, err: {}", err));
        }
    };

    if target_path.exists() {
        return Err(format!(
            "Path does not exist, path: {}",
            target_path.to_string_lossy()
        ));
    }

    let target_path_str = target_path.to_string_lossy().yellow();

    info!("Writing a config, target_path: {}", target_path_str,);

    match std::fs::write(target_path.to_owned(), serialized) {
        Ok(_) => Ok(pconfig),
        Err(err) => {
            return Err(format!(
                "Error writing pconfig to the path, err: {}",
                err
            ));
        }
    }
}

pub fn load(path: PathBuf) -> Result<PConfig, String> {
    info!(
        "Loading pconfig from path: {}",
        path.to_string_lossy().yellow()
    );

    if !path.exists() {
        return Err(format!("Path does not exist"));
    }

    let file = match std::fs::read_to_string(path.to_owned()) {
        Ok(f) => f,
        Err(err) => {
            return Err(format!("Could not read the file, err: {}", err));
        }
    };

    match serde_yaml::from_str(file.as_str()) {
        Ok(pconf) => return Ok(pconf),
        Err(err) => {
            return Err(format!("Could not deserialize pconfig, err: {}", err));
        }
    }
}

pub fn get_config_file_path(app_prefix: &String) -> Result<PathBuf, String> {
    let app_path = FS::create_or_get_app_path(app_prefix)?;
    let config_path = app_path.join(CONFIG_FILE_NAME);

    Ok(config_path)
}
