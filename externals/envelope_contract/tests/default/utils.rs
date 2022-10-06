use sak_logger::SakLogger;

pub(crate) struct EnvelopeTestUtils {}

impl EnvelopeTestUtils {
    pub fn init_test_log() {
        SakLogger::init_test_console().unwrap();
    }
}
