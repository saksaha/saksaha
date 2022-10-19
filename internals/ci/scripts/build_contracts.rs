use crate::tasks;
use crate::CIError;
use std::env::Args;

pub(crate) fn run(_args: Args) -> Result<(), CIError> {
    tasks::build_system_contracts()?;
    // tasks::build_3rd_party_contracts()?;

    Ok(())
}
