use directories::ProjectDirs;
use logger::tinfo;
use std::fs;
use std::path::PathBuf;

pub struct FS;

impl FS {
    pub fn new() -> FS {
        FS {}
    }

    pub fn create_or_get_app_path(
        app_prefix: &String,
    ) -> Result<PathBuf, String> {
        if let Some(dir) = ProjectDirs::from("com", "Saksaha", "Saksaha") {
            let app_root_path = dir.config_dir();

            if !app_root_path.exists() {
                if let Err(err) = fs::create_dir(app_root_path) {
                    return Err(format!("Cannot create dir, err: {}", err));
                }
            }

            let prefixed_app_path = app_root_path.join(app_prefix);

            if !prefixed_app_path.exists() {
                if let Err(err) = fs::create_dir(prefixed_app_path.clone()) {
                    return Err(format!("Cannot create dir, err: {}", err));
                }
            }

            return Ok(prefixed_app_path);
        } else {
            return Err(format!(
                "No valid app (config) path provided by the operating system"
            ));
        }
    }
}
