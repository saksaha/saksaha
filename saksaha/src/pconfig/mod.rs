use crate::{
    common::errors::{Error, ErrorKind},
    crypto, err_res,
};
use directories::ProjectDirs;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use logger::log;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

pub mod parse;

pub mod fs;

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
    pub fn of(config_path: Option<&str>) -> Result<PConfig, Error> {
        if let None = config_path {
            log!(DEBUG, "Config path is not given, creating a new config\n");

            match PConfig::load_from_default() {
                Err(err) => {
                    if let ErrorKind::FileNotExist = err.kind() {
                        return PConfig::new();
                    } else {
                        return Err(err);
                    }
                }
                Ok(pconf) => return Ok(pconf),
            }
        } else {
            PConfig::load(config_path.unwrap());
            err_res!("power")
        }
    }

    fn new() -> Result<PConfig, Error> {
        let sk = crypto::generate_key();
        let (sk, pk) = crypto::encode_key_pair(sk);
        let pconf = PConfig {
            p2p: PersistedP2PConfig {
                secret: sk,
                public_key: pk,
            },
        };

        match pconf.persist() {
            Ok(_) => Ok(pconf),
            Err(err) => Err(err),
        }
    }
}
