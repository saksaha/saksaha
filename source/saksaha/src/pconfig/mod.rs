use colored::Colorize;
use log::info;
use sak_crypto::ToEncodedPoint;
use sak_fs::FS;
use sak_logger::tinfo;
use sak_p2p_addr::UnknownAddr;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub secret: String,
    pub public_key_str: String,
    pub bootstrap_addrs: Option<Vec<UnknownAddr>>,
    pub p2p_port: Option<u16>,
    pub disc_port: Option<u16>,
}

impl PConfig {
    pub fn new(app_prefix: &String) -> Result<PConfig, String> {
        info!("Loading persisted config...");

        let config_file_path = get_config_file_path(app_prefix)?;

        info!(
            "Config file path is resolved, app_prefix: {}, \
                config_file_path: {:?}",
            app_prefix, config_file_path,
        );

        if config_file_path.exists() {
            info!(
                "Found a config file at the path, path: {:?}",
                config_file_path,
            );

            return PConfig::load(config_file_path);
        } else {
            info!(
                "Could not find a config file at the path. \
                    Creating a new one, path: {:?}",
                config_file_path,
            );

            let pconfig = PConfig::create_new_config();

            let pconf = match PConfig::persist(pconfig, config_file_path) {
                Ok(p) => p,
                Err(err) => {
                    return Err(err);
                }
            };

            return Ok(pconf);
        }
    }

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
                return Err(format!(
                    "Could not deserialize pconfig, err: {}",
                    err
                ));
            }
        }
    }

    fn create_new_config() -> PConfig {
        let sk = sak_crypto::generate_key();

        let (sk, pk) = sak_crypto::encode_into_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key_str: pk,
                bootstrap_addrs: None,
                p2p_port: None,
                disc_port: None,
            },
        };

        pconf
    }
}

pub fn get_config_file_path(app_prefix: &String) -> Result<PathBuf, String> {
    let app_path = FS::create_or_get_app_path(app_prefix)?;
    let config_path = app_path.join(CONFIG_FILE_NAME);

    Ok(config_path)
}
