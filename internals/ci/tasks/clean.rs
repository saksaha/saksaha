use crate::{logln, paths::Paths, CIError};

pub(crate) fn clean_prebuild() -> Result<(), CIError> {
    logln!("Clean prebuild path");

    let prebuild_path = Paths::prebuild()?;

    for file in std::fs::read_dir(prebuild_path)? {
        let f = file?;
        let file_name = f.file_name();

        if file_name == ".gitkeep" {
            // do nothing
        } else {
            std::fs::remove_file(f.path())?;
        }
    }

    Ok(())
}

pub(crate) fn clean_target() -> Result<(), CIError> {
    logln!("Clean target path");

    let target_path = Paths::curr()?.join("target");

    if target_path.exists() {
        std::fs::remove_dir_all(target_path)?;
    }

    Ok(())
}
