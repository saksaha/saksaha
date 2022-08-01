use crate::{log, CIError};

pub(crate) fn clean_prebuild() -> Result<(), CIError> {
    log!("Clean prebuild path");

    let curr_dir = std::env::current_dir()?;

    let prebuild_path = curr_dir.join("source/prebuild");
    if !prebuild_path.exists() {
        return Err(format!("prebuild path does not exist").into());
    }

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
