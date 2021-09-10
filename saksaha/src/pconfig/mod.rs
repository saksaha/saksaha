use crate::{common::errors::Error, crypto};
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
    pub fn new(path: Option<&str>) -> Result<Self, Error> {
        if let None = path {
            log!(DEBUG, "Config path is not given, creating a new config\n");

            let sk = crypto::generate_key();
            let (sk, pk) = crypto::encode_key_pair(sk);
            let pconf = PConfig {
                p2p: PersistedP2PConfig {
                    secret: sk,
                    public_key: pk,
                },
            };
            pconf.persist();

            let serialized = serde_json::to_string(&pconf).unwrap();
            print!("{}\n", serialized);
            // match serialized {
            //    Ok(s) => {return Ok(pconf)},
            //    _ => {return Error::result(format!("f"))}
            // }
        } else {
            PConfig::load(path.unwrap());
        }

        // let pk = sk.public_key();

        // crypto::encode_hex(sk.to_bytes().as_slice());
        // let bb = pk.to_encoded_point(false);

        // crypto::encode_hex(pk.as_affine());
        return Error::result(format!("f"));
    }
}
