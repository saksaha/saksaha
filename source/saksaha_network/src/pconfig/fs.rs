use crate::{pconfig::PConfig, SaksahaError};
use log::info;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";

pub fn get_config_file_path(
    app_prefix: &String,
) -> Result<PathBuf, SaksahaError> {
    let db_name = sak_fs::DBName::Saksaha;
    let app_path = sak_fs::create_or_get_app_path(db_name, app_prefix)?;

    let config_path = app_path.join(CONFIG_FILE_NAME);

    Ok(config_path)
}
