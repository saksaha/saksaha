use crate::CIError;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

static PATHS: OnceCell<Paths> = OnceCell::new();

#[derive(Debug)]
pub struct Paths {
    prebuild: PathBuf,
    external: PathBuf,
    source: PathBuf,
    curr: PathBuf,
}

impl Paths {
    pub fn init(curr_dir: PathBuf) -> Result<(), CIError> {
        let prebuild = curr_dir.join("source/prebuild");
        if !prebuild.exists() {
            return Err(format!("prebuild path does not exist").into());
        }

        let external = curr_dir.join("extern");
        if !external.exists() {
            return Err(format!("external path does not exist").into());
        }

        let source = curr_dir.join("source");
        if !source.exists() {
            return Err(format!("source path does not exist").into());
        }

        let paths = Paths {
            prebuild,
            external,
            source,
            curr: curr_dir,
        };

        match PATHS.set(paths) {
            Ok(_) => (),
            Err(_err) => return Err(format!("Cannot initialize Paths").into()),
        };

        Ok(())
    }

    pub fn curr() -> Result<&'static PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        Ok(&paths.curr)
    }

    pub fn source() -> Result<&'static PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        Ok(&paths.source)
    }

    pub fn external() -> Result<&'static PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        Ok(&paths.external)
    }

    pub fn prebuild() -> Result<&'static PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        Ok(&paths.prebuild)
    }
}
