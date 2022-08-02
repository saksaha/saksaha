use crate::{log, tasks, CIError};
use std::{env::Args, process::Command as Cmd};

pub(crate) fn run(_args: Args) -> Result<(), CIError> {
    tasks::clean_prebuild()?;
    tasks::clean_target()?;

    Ok(())
}
