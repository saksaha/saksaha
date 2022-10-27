use crate::{tasks, CIError};
use std::env::Args;

pub(crate) fn run(_args: Args) -> Result<(), CIError> {
    tasks::clean_prebuild()?;
    tasks::clean_target()?;

    Ok(())
}
