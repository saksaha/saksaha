use crate::tasks;
use crate::CIError;
use std::env::Args;

pub(crate) fn run(_args: Args) -> Result<(), CIError> {
    tasks::build_circuit_params_2_to_2()?;

    Ok(())
}
