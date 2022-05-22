use directories::ProjectDirs;
use logger::tinfo;
use std::fs;
use std::path::PathBuf;

pub struct FS;

impl FS {
    pub fn new() -> FS {
        FS {}
    }

    pub fn create_or_get_app_path() -> Result<PathBuf, String> {
        if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
            let app_path = dir.config_dir();
            if !app_path.exists() {
                match fs::create_dir(app_path) {
                    Ok(_) => {
                        return Ok(app_path.to_path_buf());
                    }
                    Err(err) => {
                        return Err(format!("Cannot create dir, err: {}", err));
                    }
                }
            }
            return Ok(app_path.to_path_buf());
        } else {
            return Err(format!(
                "No valid app (config) path provided by the operating system"
            ));
        }
    }
}
