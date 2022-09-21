use crate::FSError;
use std::path::PathBuf;

pub fn persist(data: &String, target_path: &PathBuf) -> Result<(), FSError> {
    match std::fs::write(target_path.to_owned(), data) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(format!("Error writing pconfig to the path, err: {}", err).into());
        }
    }
}

pub fn load(path: &PathBuf) -> Result<Vec<u8>, FSError> {
    if !path.exists() {
        return Err(format!("Path does not exist").into());
    }

    let file = match std::fs::read(path.to_owned()) {
        Ok(f) => f,
        Err(err) => {
            return Err(format!("Could not read the file, err: {}", err).into());
        }
    };

    Ok(file)
}
