use crate::CIError;
use once_cell::sync::OnceCell;
use std::path::PathBuf;

static PATHS: OnceCell<Paths> = OnceCell::new();

#[derive(Debug)]
pub struct Paths {
    project_root: PathBuf,
}

impl Paths {
    pub fn init(project_root: PathBuf) -> Result<(), CIError> {
        let paths = Paths { project_root };

        match PATHS.set(paths) {
            Ok(_) => (),
            Err(_err) => return Err(format!("Cannot initialize Paths").into()),
        };

        Ok(())
    }

    pub fn project_root() -> Result<PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        Ok(paths.project_root.clone())
    }

    pub fn source() -> Result<PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        let source = paths.project_root.join("source");
        if !source.exists() {
            return Err(format!("source path does not exist").into());
        }

        Ok(source)
    }

    pub fn externals() -> Result<PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        let externals = paths.project_root.join("externals");
        if !externals.exists() {
            return Err(format!("externals path does not exist").into());
        }

        Ok(externals)
    }

    pub fn prebuild() -> Result<PathBuf, CIError> {
        let paths = PATHS.get().ok_or("Paths should have been initialized")?;

        let prebuild = paths.project_root.join("source/prebuild");
        if !prebuild.exists() {
            return Err(format!("prebuild path does not exist").into());
        }

        Ok(prebuild)
    }
}
