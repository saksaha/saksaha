use colored::Colorize;
use directories::ProjectDirs;
use log::info;
use std::fs;
use std::path::PathBuf;

pub type FSError = Box<dyn std::error::Error + Send + Sync>;

pub fn get_app_root_path(app_name: &str) -> Result<PathBuf, FSError> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", app_name) {
        let app_root_path = dir.config_dir();

        if !app_root_path.exists() {
            fs::create_dir(app_root_path)?;
        }

        return Ok(app_root_path.to_path_buf());
    } else {
        return Err(format!(
            "No valid app (config) path provided by the operating system"
        )
        .into());
    }
}

// {home}/{config}/{app_name}/{app_prefix}/...
pub fn create_or_get_app_path(
    app_name: &str,
    // app_prefix: &String,
) -> Result<PathBuf, FSError> {
    if let Some(dir) = ProjectDirs::from("com", "Saksaha", app_name) {
        let app_root_path = dir.config_dir();

        if !app_root_path.exists() {
            fs::create_dir(app_root_path)?;
        }

        // let prefixed_app_path = app_root_path.join(app_prefix);

        // if !prefixed_app_path.exists() {
        //     if let Err(err) = fs::create_dir(prefixed_app_path.clone()) {
        //         return Err(format!("Cannot create dir, err: {}", err).into());
        //     }
        // }

        return Ok(app_root_path.to_path_buf());
    } else {
        return Err(format!(
            "No valid app (config) path provided by the operating system"
        )
        .into());
    }
}

pub fn persist(data: String, target_path: PathBuf) -> Result<(), FSError> {
    let target_path_str = target_path.to_string_lossy().yellow();

    info!("Writing a config, target_path: {}", target_path_str,);

    match std::fs::write(target_path.to_owned(), data) {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err(format!(
                "Error writing pconfig to the path, err: {}",
                err
            )
            .into());
        }
    }
}

pub fn load(path: PathBuf) -> Result<Vec<u8>, FSError> {
    info!(
        "Loading pconfig from path: {}",
        path.to_string_lossy().yellow()
    );

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
