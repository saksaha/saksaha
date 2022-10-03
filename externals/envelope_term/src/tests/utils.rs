use crate::{fs::FS, EnvelopeError};
use sak_logger::SakLogger;

pub(crate) struct EnvelopeTermTestUtils;

impl EnvelopeTermTestUtils {
    pub fn init_test() -> Result<(), EnvelopeError> {
        // let config_dir = FS::config_dir()?;

        SakLogger::init_test_console();

        Ok(())
    }
}
