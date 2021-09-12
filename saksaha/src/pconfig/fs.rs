use super::PConfig;
use crate::common::errors::{Error, ErrorKind};
use crate::{err_res, err_resk};
use directories::ProjectDirs;
use logger::log;
use std::fs;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.json";

impl PConfig {
    pub fn persist(&self) -> Result<PathBuf, Error> {
        let serialized = serde_json::to_string_pretty(&self).unwrap();
        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(CONFIG_FILE_NAME).to_owned();

        if config_path.exists() {
            return err_res!("Config file already exists, something is wrong");
        }

        log!(DEBUG, "Writing a config, at: {:?}\n", config_path);

        match fs::write(config_path.to_owned(), serialized) {
            Ok(_) => Ok(config_path),
            Err(err) => err_res!("Error writing the config, err: {}", err),
        }
    }

    pub fn load(config_path: PathBuf) -> Result<PConfig, Error> {
        log!(DEBUG, "Load configuration, path: {:?}\n", config_path);

        return Self::load_config(config_path);
    }

    fn load_config(path: PathBuf) -> Result<PConfig, Error> {
        if !path.exists() {
            return err_resk!(
                ErrorKind::FileNotExist,
                "Config does not exist at path: {:?}\n",
                path
            );
        }

        let f = fs::read_to_string(path.to_owned());

        if let Err(err) = f {
            return err_res!(
                "Error reading file, path: {:?}, err: {}",
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

    pub fn get_default_path() -> Result<PathBuf, Error> {
        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(CONFIG_FILE_NAME);
        Ok(config_path)
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
