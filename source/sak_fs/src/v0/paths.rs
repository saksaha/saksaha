use crate::FSError;
use directories::ProjectDirs;
use std::fs;
use std::path::PathBuf;

pub fn get_config_dir(app_name: &str) -> Result<PathBuf, FSError> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", app_name) {
        let config_dir = dir.config_dir();

        if !config_dir.exists() {
            fs::create_dir(config_dir)?;
        }

        return Ok(config_dir.to_path_buf());
    } else {
        return Err(format!(
            "No valid app (config) path provided by the operating system"
        )
        .into());
    }
}
