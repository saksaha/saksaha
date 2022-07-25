use super::fs;
use crate::EnvelopeError;
use directories::ProjectDirs;
use log::info;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_logger::tinfo;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    // public_key: String,
    // secret: String,
}

impl PConfig {
    // pub fn new() -> Self {
    //     PConfig {
    //         public_key: "power".into(),
    //         secret: "secret".into(),
    //     }
    // }

    pub fn load(path: Option<PathBuf>) -> Self {
        // HOME_PATH/CONFIG_PATH/ENVELOPE_PATH/{name-space}/config.yaml
        // create_or_get_app_path()

        // let path = path.unwrap_or(PathBuf)

        // info!(
        //     "Loading pconfig from path: {}",
        //     path.to_string_lossy().yellow()
        // );

        // if !path.exists() {
        //     return Err(format!("Path does not exist"));
        // }

        // let file = match std::fs::read_to_string(path.to_owned()) {
        //     Ok(f) => f,
        //     Err(err) => {
        //         return Err(format!("Could not read the file, err: {}", err));
        //     }
        // };

        // match serde_yaml::from_str(file.as_str()) {
        //     Ok(pconf) => return Ok(pconf),
        //     Err(err) => {
        //         return Err(format!("Could not deserialize pconfig, err: {}", err));
        //     }
        // }

        PConfig {
            // public_key: "power".into(),
            // secret: "secret".into(),
        }
    }

    pub fn new(app_prefix: &String) -> Result<PConfig, EnvelopeError> {
        info!("Loading persisted config...");

        let config_file_path = fs::get_config_file_path(app_prefix)?;

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

            let file = sak_fs::load(config_file_path)?;

            let pconfig = serde_yaml::from_slice::<PConfig>(&file)?;

            Ok(pconfig)
        } else {
            info!(
                "Could not find a config file at the path. \
                    Creating a new one, path: {:?}",
                config_file_path,
            );

            let pconfig = PConfig::create_new_config();

            let data = serde_yaml::to_string(&pconfig)?;

            sak_fs::persist(data, config_file_path)?;

            return Ok(pconfig);
        }
    }

    fn create_new_config() -> PConfig {
        let (sk, pk) = SakKey::generate();

        let secret_str = sak_crypto::encode_hex(&sk.to_bytes());
        let public_key_str =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let pconf = PConfig {};

        pconf
    }
}

fn create_or_get_app_path(app_prefix: &String) -> Result<PathBuf, String> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Envelope") {
        let app_root_path = dir.config_dir();

        if !app_root_path.exists() {
            if let Err(err) = std::fs::create_dir(app_root_path) {
                return Err(format!("Cannot create dir, err: {}", err));
            }
        }

        let prefixed_app_path = app_root_path.join(app_prefix);

        if !prefixed_app_path.exists() {
            if let Err(err) = std::fs::create_dir(prefixed_app_path.clone()) {
                return Err(format!("Cannot create dir, err: {}", err));
            }
        }

        return Ok(prefixed_app_path);
    } else {
        return Err(format!(
            "No valid app (config) path provided by the operating system"
        ));
    }
}
