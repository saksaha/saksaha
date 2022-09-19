use crate::FSError;
use colored::Colorize;
use directories::ProjectDirs;
use log::info;
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

// // {home}/{config}/{app_name}/{app_prefix}/...
// pub fn create_or_get_app_path(app_name: &str) -> Result<PathBuf, FSError> {
//     if let Some(dir) = ProjectDirs::from("com", "Saksaha", app_name) {
//         let app_root_path = dir.config_dir();

//         if !app_root_path.exists() {
//             fs::create_dir_all(app_root_path)?;
//         }

//         return Ok(app_root_path.to_path_buf());
//     } else {
//         return Err(format!(
//             "No valid app (config) path provided by the operating system"
//         )
//         .into());
//     }
// }
