use crate::fs;
use crate::SaksahaError;
use std::path::PathBuf;

const CONFIG_FILE_NAME: &str = "config.yml";

// pub fn get_acc_dir(public_key: &String) -> Result<PathBuf, SaksahaError> {
//     let acc_dir = fs::acc_dir(public_key)?;

//     Ok(acc_dir)
// }

pub fn get_config_file_path(public_key: &String) -> Result<PathBuf, SaksahaError> {
    let acc_dir = fs::acc_dir(public_key)?;

    let config_file_path = acc_dir.join(CONFIG_FILE_NAME);

    Ok(config_file_path)
}
