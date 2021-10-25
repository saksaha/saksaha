pub mod error;
pub mod fs;

use log::{info, debug};
use crypto::Crypto;
use fs::FS;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use self::error::PConfigError;

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
    pub fn from_path(
        config_path: Option<String>,
    ) -> Result<PConfig, PConfigError> {
        let config_path = match config_path {
            Some(c) => c,
            None => {
                info!(
                    "Config path is not given, creating a new config"
                );

                let default_path = FS::get_default_path()?;

                if default_path.exists() {
                    info!("Found a config at the default location");

                    return FS::load(default_path);
                } else {
                    let pconfig = PConfig::new();

                    let pconf = match FS::persist(pconfig) {
                        Ok(p) => p,
                        Err(err) => {
                            return Err(PConfigError::PersistError);
                        }
                    };

                    return Ok(pconf);
                }
            }
        };

        let config_path = PathBuf::from(config_path);
        FS::load(config_path)
    }

    fn new() -> PConfig {
        debug!("Creating a new config");

        let sk = Crypto::generate_key();
        let (sk, pk) = Crypto::encode_into_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key: pk,
            },
        };

        pconf
    }
}
