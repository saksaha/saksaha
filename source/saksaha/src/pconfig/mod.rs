mod config;
pub mod error;
pub mod fs;

use self::error::PConfigError;
use crate::p2p::identity::Identity;
use colored::Colorize;
pub use config::*;
use fs::FS;
use logger::tinfo;
use p2p_identity::peer::UnknownPeer;
use std::path::PathBuf;

impl PConfig {
    pub fn from_path(
        config_path: Option<String>,
    ) -> Result<PConfig, PConfigError> {
        tinfo!("saksaha", "pconfig", "",);
        tinfo!("saksaha", "pconfig", "Loading persisted config...");

        let config_path = match config_path {
            Some(c) => c,
            None => {
                let default_path = FS::get_default_path()?;
                tinfo!(
                    "saksaha",
                    "pconfig",
                    "Config path is not given. Defaults to location, {:?}",
                    default_path,
                );

                if default_path.exists() {
                    tinfo!(
                        "saksaha",
                        "pconfig",
                        "Found a config at the default location, path: {:?}",
                        default_path,
                    );

                    return FS::load(default_path);
                } else {
                    tinfo!(
                        "saksaha",
                        "pconfig",
                        "Couldn't find a config file at the default path. \
                        Creating a new one, path: {:?}",
                        default_path,
                    );

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
        tinfo!("saksaha", "", "Creating a new config".yellow());

        let sk = crypto::generate_key();
        let (sk, pk) = crypto::encode_into_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                identity: Identity {
                    secret: sk,
                    public_key: pk,
                },
                bootstrap_addrs: None,
                p2p_port: None,
                disc_port: None,
            },
        };

        pconf
    }
}
