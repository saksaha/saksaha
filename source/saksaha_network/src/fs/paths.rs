use super::SaksahaFSError;
use std::path::PathBuf;

const APP_NAME: &str = "saksaha";

pub fn config_dir() -> Result<PathBuf, SaksahaFSError> {
    // if let Some(dir) = ProjectDirs::from("com", "Saksaha", APP_NAME) {
    //     let app_root_path = dir.config_dir();

    //     if !app_root_path.exists() {
    //         std::fs::create_dir(app_root_path)?;
    //     }

    //     return Ok(app_root_path.to_path_buf());
    // } else {
    //     return Err(format!(
    //         "No valid app (config) path provided by the operating system"
    //     )
    //     .into());
    // }

    sak_fs::get_config_dir(APP_NAME)
}

pub fn acc_dir(public_key: &String) -> Result<PathBuf, SaksahaFSError> {
    let p = sak_fs::get_config_dir(APP_NAME)?.join(public_key);
    Ok(p)
}
