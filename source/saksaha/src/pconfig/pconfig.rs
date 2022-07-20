use colored::Colorize;
use log::info;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_fs::FS;
use sak_logger::tinfo;
use sak_p2p_addr::UnknownAddr;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::pconfig::fs;

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
    pub fn new(app_prefix: &String) -> Result<PConfig, String> {
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

            return fs::load(config_file_path);
        } else {
            info!(
                "Could not find a config file at the path. \
                    Creating a new one, path: {:?}",
                config_file_path,
            );

            let pconfig = PConfig::create_new_config();

            let pconf = match fs::persist(pconfig, config_file_path) {
                Ok(p) => p,
                Err(err) => {
                    return Err(err);
                }
            };

            return Ok(pconf);
        }
    }

    fn create_new_config() -> PConfig {
        let (sk, pk) = SakKey::generate();

        println!("power33333333333: {:?}", sk.to_bytes());

        let secret_str = sak_crypto::encode_hex(&sk.to_bytes());
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
