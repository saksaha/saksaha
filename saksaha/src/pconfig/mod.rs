use crate::errors::Error;
use directories::ProjectDirs;
use logger::log;
use std::path::{Path, PathBuf};

pub mod parse;

pub mod fs;

pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

pub struct PersistedP2PConfig {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl PConfig {
    pub fn new(path: Option<&str>) -> Result<Self, Error> {
        return Error::result(format!("f"));
    }
}
