pub mod fs;

use crate::{common::Result, crypto::Crypto, err};
use fs::FS;
use logger::log;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub secret: String,
    pub public_key: String,
}

impl PConfig {
    pub fn from_path(config_path: Option<String>) -> Result<PConfig> {
        let config_path = match config_path {
            Some(c) => c,
            None => {
                log!(
                    DEBUG,
                    "Config path is not given, creating a new config\n"
                );

                let default_path = FS::get_default_path()?;

                if default_path.exists() {
                    log!(DEBUG, "Found a config at the default location\n");

                    return FS::load(default_path);
                } else {
                    let pconfig = match PConfig::new() {
                        Ok(p) => p,
                        Err(err) => {
                            return err!(
                                "Error initializing pconfig, err: {}",
                                err
                            );
                        }
                    };

                    let pconf = match FS::persist(pconfig) {
                        Ok(p) => p,
                        Err(err) => {
                            return err!(
                                "Cannot persist pconfig, err: {}",
                                err
                            );
                        }
                    };

                    return Ok(pconf);
                }
            }
        };

        let config_path = PathBuf::from(config_path);
        FS::load(config_path)
    }

    fn new() -> Result<PConfig> {
        log!(DEBUG, "Creating a new config\n");

        let sk = Crypto::generate_key();
        let (sk, pk) = Crypto::encode_into_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key: pk,
            },
        };

        Ok(pconf)
    }
}
