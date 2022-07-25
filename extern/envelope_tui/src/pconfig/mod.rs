use std::path::PathBuf;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    public_key: String,
    secret: String,
}

impl PConfig {
    pub fn new() -> Self {
        PConfig {
            public_key: "power".into(),
            secret: "secret".into(),
        }
    }

    pub fn load(path: Option<PathBuf>) -> Self {
        // HOME_PATH/CONFIG_PATH/ENVELOPE_PATH/{name-space}/config.yaml

        // let path = path.unwrap_or(PathBuf)

        // info!(
        //     "Loading pconfig from path: {}",
        //     path.to_string_lossy().yellow()
        // );

        // if !path.exists() {
        //     return Err(format!("Path does not exist"));
        // }

        // let file = match std::fs::read_to_string(path.to_owned()) {
        //     Ok(f) => f,
        //     Err(err) => {
        //         return Err(format!("Could not read the file, err: {}", err));
        //     }
        // };

        // match serde_yaml::from_str(file.as_str()) {
        //     Ok(pconf) => return Ok(pconf),
        //     Err(err) => {
        //         return Err(format!("Could not deserialize pconfig, err: {}", err));
        //     }
        // }

        PConfig {
            public_key: "power".into(),
            secret: "secret".into(),
        }
    }
}
