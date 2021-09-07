use crate::errors::Error;
use directories::ProjectDirs;
use logger::log;
use std::path::{Path, PathBuf};

pub mod path;

pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

pub struct PersistedP2PConfig {
    pub private_key: Option<String>,
    pub public_key: Option<String>,
}

impl PConfig {
    pub fn new(path: Option<&str>) -> Self {
        load_or_create_config(path);

        PConfig {
            p2p: PersistedP2PConfig {
                private_key: None,
                public_key: None,
            },
        }
    }
}

fn load_or_create_config(path: Option<&str>) -> Result<PathBuf, Error> {
    if let Some(p) = path {
        log!(DEBUG, "Config path is given, probing: {}\n", p);

        let path = PathBuf::from(p);

        if path.exists() {
            log!(DEBUG, "Found config file, loading...\n");
            return Ok(path);
        } else {
            return Error::result(format!("Config file does not exist"));
        }
    } else {
        log!(
            DEBUG,
            "Config path is not given, creating the default one\n"
        );

        let path = get_default_path();

        let err =
            Error::result(format!("Error creating a default config path"));

        match path {
            Some(p) => {
                if let Ok(_) = create_path(p.as_path()) {
                    return Ok(p);
                } else {
                    return err;
                }
            }
            None => return err,
        }
    }
}

fn create_path(p: &Path) -> std::io::Result<()> {
    return std::fs::create_dir(p);
}

fn get_default_path() -> Option<PathBuf> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        return Some(dir.config_dir().to_path_buf());
    } else {
        return None;
    }
}

#[cfg(test)]
mod test {
    use crate::testenv;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn it_creates_default_config_path() {
        testenv::run_test(|test_env| {
            let path = Some("saksaha-config");
            super::load_or_create_config(path);

            let testdump = test_env.testdump.as_ref().unwrap();

            println!("{:?}", testdump);
        })
    }
}
