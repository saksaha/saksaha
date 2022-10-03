use sak_logger::SakLogger;

use crate::{fs::FS, EnvelopeError};

pub(crate) fn init_test() -> Result<(), EnvelopeError> {
    let config_dir = FS::config_dir()?;

    SakLogger::init_test(config_dir);

    Ok(())
}
