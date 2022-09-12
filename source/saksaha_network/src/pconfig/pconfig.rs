use crate::{pconfig::fs, SaksahaError};
use colored::Colorize;
use log::info;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_p2p_addr::UnknownAddr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub secret: String,
    pub public_key: String,
    pub acc_addr: String,
    pub bootstrap_addrs: Option<Vec<UnknownAddr>>,
    pub p2p_port: Option<u16>,
    pub disc_port: Option<u16>,
}

impl PConfig {
    pub fn new(public_key: &Option<String>) -> Result<PConfig, SaksahaError> {
        info!("Loading persisted config...");

        let config_file_path = fs::get_config_file_path(public_key)?;

        info!(
            "Config file path is resolved, public_key: {}, \
                config_file_path: {:?}",
            public_key, config_file_path,
        );

        if config_file_path.exists() {
            info!(
                "Found a config file at the path, path: {:?}",
                config_file_path,
            );

            let data = sak_fs::load(config_file_path)?;

            let pconfig = serde_yaml::from_slice::<PConfig>(&data)?;

            Ok(pconfig)
        } else {
            info!(
                "Could not find a config file at the path. \
                    Creating a new one, path: {:?}",
                config_file_path,
            );

            let pconfig = PConfig::create_new_config();

            let data = serde_yaml::to_string(&pconfig)?;

            let config_path = fs::get_config_path(public_key)?;

            let _ = std::fs::create_dir_all(config_path);

            sak_fs::persist(data, config_file_path)?;

            Ok(pconfig)
        }
    }

    fn create_new_config() -> PConfig {
        let (sk, pk) = SakKey::generate();

        let secret_str = sak_crypto::encode_hex(&sk.to_bytes() as &[u8]);
        let public_key_str =
            sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: secret_str,
                public_key: public_key_str,
                acc_addr,
                bootstrap_addrs: None,
                p2p_port: None,
                disc_port: None,
            },
        };

        pconf
    }
}
