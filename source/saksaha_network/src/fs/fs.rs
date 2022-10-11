use super::SaksahaFSError;
use std::path::PathBuf;

const APP_NAME: &str = "saksaha";

const CONFIG_FILE_NAME: &str = "config.yml";

pub(crate) struct SaksahaFS;

impl SaksahaFS {
    pub fn config_dir() -> Result<PathBuf, SaksahaFSError> {
        sak_dir::get_config_dir(APP_NAME)
    }

    pub fn acc_dir(public_key: &String) -> Result<PathBuf, SaksahaFSError> {
        let p = sak_dir::get_config_dir(APP_NAME)?.join(public_key);
        Ok(p)
    }

    pub fn get_config_file_path(public_key: &String) -> Result<PathBuf, SaksahaFSError> {
        let acc_dir = SaksahaFS::acc_dir(public_key)?;

        let config_file_path = acc_dir.join(CONFIG_FILE_NAME);

        Ok(config_file_path)
    }
}
