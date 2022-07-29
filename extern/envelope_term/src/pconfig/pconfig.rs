use super::fs;
use crate::EnvelopeError;
use directories::ProjectDirs;
use log::{info, warn};
use sak_crypto::{SakKey, ToEncodedPoint};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    user_name: String,
    public_key: String,
    secret: String,
    key_storage: HashMap<String, [u8; 32]>,
}

impl PConfig {
    pub fn new(app_prefix: &String) -> Result<PConfig, EnvelopeError> {
        info!("Loading persisted config...");

        let config_file_path = fs::get_config_file_path(app_prefix)?;

        info!(
            "Config file path is resolEved, app_prefix: {}, \
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

            let mut pconfig = PConfig::create_new_config();
            pconfig.user_name = app_prefix.clone();

            let data = serde_yaml::to_string(&pconfig)?;

            sak_fs::persist(data, config_file_path)?;

            return Ok(pconfig);
        }
    }

    pub fn insert_ch_key(
        &mut self,
        ch_id: String,
        key: [u8; 32],
    ) -> Result<(), EnvelopeError> {
        self.key_storage.insert(ch_id, key);
        let user_name = self.user_name.clone();
        let config_file_path = fs::get_config_file_path(&user_name)?;
        let data = serde_yaml::to_string(&self)?;
        sak_fs::persist(data, config_file_path)?;
        return Ok(());
    }

    pub fn get_ch_key(
        &self,
        ch_id: &String,
    ) -> Result<[u8; 32], EnvelopeError> {
        let v = match self.key_storage.get(ch_id) {
            Some(v) => v,
            None => {
                return Err(
                    format!("no matching epherial key with ch_id").into()
                );
            }
        };
        Ok(*v)
    }

    pub fn load(app_prefix: &String) -> Result<Self, EnvelopeError> {
        let path = fs::get_config_file_path(app_prefix)?;

        // info!(
        //     "Loading pconfig from path: {}",
        //     path.to_string_lossy().yellow()
        // );

        if !path.exists() {
            warn!("Path does not exist");
        }

        let file = match std::fs::read_to_string(path.to_owned()) {
            Ok(f) => f,
            Err(err) => {
                return Err(
                    format!("Could not read the file, err: {}", err).into()
                );
            }
        };

        match serde_yaml::from_str(file.as_str()) {
            Ok(pconf) => return Ok(pconf),
            Err(err) => {
                return Err(format!(
                    "Could not deserialize pconfig, err: {}",
                    err
                )
                .into());
            }
        }
    }

    fn create_new_config() -> PConfig {
        let (sk, pk) = SakKey::generate();

        let secret_str = sak_crypto::encode_hex(&sk.to_bytes());

        let public_key_str =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        // let acc_addr = SakKey::create_acc_addr(&pk);
        let key_storage = HashMap::new();

        let pconf = PConfig {
            user_name: String::new(),
            public_key: public_key_str,
            secret: secret_str,
            key_storage,
        };

        pconf
    }

    pub fn get_sk_pk(&self) -> (String, String) {
        (self.secret.clone(), self.public_key.clone())
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
