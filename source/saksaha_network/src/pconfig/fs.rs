use crate::SaksahaError;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";

pub fn get_config_path(public_key: &String) -> Result<PathBuf, SaksahaError> {
    let app_path = sak_fs::get_app_root_path("saksaha")?.join(public_key);

    Ok(app_path)
}

pub fn get_config_file_path(
    app_prefix: &String,
) -> Result<PathBuf, SaksahaError> {
    let app_path = sak_fs::get_app_root_path("saksaha")?.join(app_prefix);

    let config_path = app_path.join(CONFIG_FILE_NAME);

    Ok(config_path)
}
