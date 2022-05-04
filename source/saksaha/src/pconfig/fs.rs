use crate::pconfig::error::PConfigError;
use crate::pconfig::PConfig;
use directories::ProjectDirs;
use logger::tinfo;
use std::fs;
use std::path::PathBuf;

const DEFAULT_CONFIG_FILE: &str = "config.yml";

pub struct FS;

impl FS {
    pub fn new() -> FS {
        FS {}
    }

    pub fn persist(pconfig: PConfig) -> Result<PConfig, PConfigError> {
        let serialized = match serde_yaml::to_string(&pconfig) {
            Ok(s) => s,
            Err(err) => {
                return Err(PConfigError::SerializationFail {
                    err: err.to_string(),
                });
            }
        };

        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(DEFAULT_CONFIG_FILE).to_owned();

        if config_path.exists() {
            return Err(PConfigError::PathNotFound(config_path));
        }

        tinfo!(
            "saksaha",
            "pconfig",
            "Writing a config, at: {:?}",
            config_path,
        );

        match fs::write(config_path.to_owned(), serialized) {
            Ok(_) => Ok(pconfig),
            Err(err) => Err(PConfigError::ConfigWriteFail {
                err: err.to_string(),
            }),
        }
    }

    pub fn load(path: PathBuf) -> Result<PConfig, PConfigError> {
        tinfo!(
            "saksaha",
            "pconfig",
            "Loading configuration at path: {:?}",
            path
        );

        if !path.exists() {
            return Err(PConfigError::PathNotFound(path));
        }

        let file = match fs::read_to_string(path.to_owned()) {
            Ok(f) => f,
            Err(err) => {
                return Err(PConfigError::ReadFail {
                    err: err.to_string(),
                });
            }
        };

        match serde_yaml::from_str(file.as_str()) {
            Ok(pconf) => return Ok(pconf),
            Err(err) => {
                return Err(PConfigError::DeserializationFail {
                    err: err.to_string(),
                });
            }
        }
    }

    pub fn get_default_path() -> Result<PathBuf, PConfigError> {
        let app_path = create_or_get_app_path()?;
        let config_path = app_path.join(DEFAULT_CONFIG_FILE);

        Ok(config_path)
    }
}

fn create_or_get_app_path() -> Result<PathBuf, PConfigError> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
        let app_path = dir.config_dir();
        if !app_path.exists() {
            match fs::create_dir(app_path) {
                Ok(_) => {
                    return Ok(app_path.to_path_buf());
                }
                Err(err) => {
                    return Err(PConfigError::PathCreationFail {
                        err: err.to_string(),
                    });
                }
            }
        }
        return Ok(app_path.to_path_buf());
    } else {
        return Err(PConfigError::PathCreationFail {
            err: "couldn't form the right".into(),
        });
    }
}
