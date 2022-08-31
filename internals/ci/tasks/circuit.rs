use crate::{log, paths::Paths, CIError};

pub(crate) fn build_circuit_params() -> Result<(), CIError> {
    log!("Build circuit params");

    // let prebuild_path = Paths::prebuild()?;

    // for file in std::fs::read_dir(prebuild_path)? {
    //     let f = file?;
    //     let file_name = f.file_name();

    //     if file_name == ".gitkeep" {
    //         // do nothing
    //     } else {
    //         std::fs::remove_file(f.path())?;
    //     }
    // }

    Ok(())
}
