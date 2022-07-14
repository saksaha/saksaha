use colored::Colorize;
use log::info;
use sak_crypto::ToEncodedPoint;
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
    pub public_key_str: String,
    // pub addr_pk: String,
    // pub addr_sk: String,
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
        let sk = sak_crypto::generate_key();

        let (sk, pk) = sak_crypto::encode_into_key_pair(sk);

        let acc_addr = sak_crypto::create_acc_addr(&pk);

        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key_str: pk,
                // acc_addr,
                bootstrap_addrs: None,
                p2p_port: None,
                disc_port: None,
            },
        };

        pconf
    }
}
