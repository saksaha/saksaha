pub mod error;
pub mod fs;

use self::error::PConfigError;
use fs::FS;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use logger::{tinfo};

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
        tinfo!("sak", "loading persisted config...");

        let config_path = match config_path {
            Some(c) => c,
            None => {
                let default_path = FS::get_default_path()?;
                tinfo!(
                    "sak",
                    "Config path is not given. Defaults to location, {:?}",
                    default_path,
                );

                if default_path.exists() {
                    tinfo!(
                        "sak",
                        "Found a config at the default location, path: {:?}",
                        default_path,
                    );

                    return FS::load(default_path);
                } else {
                    tinfo!("sak", "Config path ");

                    let pconfig = PConfig::new();

                    let pconf = match FS::persist(pconfig) {
                        Ok(p) => p,
                        Err(_) => {
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
        tinfo!("sak", "Creating a new config");

        let sk = crypto::generate_key();
        let (sk, pk) = crypto::encode_into_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key: pk,
            },
        };

        pconf
    }
}
