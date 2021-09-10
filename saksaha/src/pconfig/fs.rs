use super::{parse, PConfig};
use crate::{err_res, err_resk};
use crate::common::errors::{Error, ErrorKind};
use directories::ProjectDirs;
use logger::log;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const CONFIG_FILE_NAME: &str = "config.json";

impl PConfig {
    pub fn persist(&self) -> Result<PathBuf, Error> {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(CONFIG_FILE_NAME);

        if config_path.exists() {
            return err_res!("Config file already exists, something is wrong");
        }

        match fs::write(config_path.to_owned(), serialized) {
            Ok(_) => Ok(config_path),
            Err(err) => err_res!("Error writing the config, err: {}", err),
        }
    }

    pub fn load(config_path: &str) -> Result<PConfig, Error> {
        return Self::load_config(config_path);
    }

    pub fn load_from_default() -> Result<PConfig, Error> {
        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(CONFIG_FILE_NAME);
        let config_path = config_path.to_str()
            .expect("config path must be properly constructed");

        return PConfig::load_config(config_path);
    }

    fn load_config(path: &str) -> Result<PConfig, Error> {
        if !PathBuf::from(path).exists() {
            return err_resk!(ErrorKind::FileNotExist, "");
        }

        let f = fs::read_to_string(path);

        if let Err(err) = f {
            return err_res!(
                "Error reading file, path: {}, err: {}",
                path,
                err
            );
        }

        match serde_json::from_str(f.unwrap().as_str()) {
            Ok(pconf) => return Ok(pconf),
            Err(err) => {
                return err_res!("Error deserializing config, err: {}", err);
            }
        }
    }
}

fn create_or_get_app_path() -> Result<PathBuf, Error> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        let app_path = dir.config_dir();
        if !app_path.exists() {
            match fs::create_dir(app_path) {
                Ok(_) => {
                    return Ok(app_path.to_path_buf());
                }
                Err(err) => {
                    return err_res!("Error creating a path, {}", err);
                }
            }
        }
        return Ok(app_path.to_path_buf());
    } else {
        return err_res!("Error forming an app path");
    }
}

#[cfg(test)]
mod test {
    use crate::testenv;

    #[test]
    fn it_creates_config_path() {
        testenv::run_test(|test_env| {
            let testdump = test_env
                .testdump
                .as_ref()
                .expect("Test dump path should be provided");

            let path = testdump.join("saksaha-config");
            let path = path.to_str().expect("Error making test config path");

            // let _ = super::load_or_create_config(Some(path))
            //     .expect("Error creating config");

            // PathBuf::from(path_name);

            println!("{:?}", testdump);
        })
    }
}
