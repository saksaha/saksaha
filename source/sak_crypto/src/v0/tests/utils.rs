use sak_logger::SakLogger;

pub(crate) struct SakCryptoTestUtils;

impl SakCryptoTestUtils {
    pub fn init_test() {
        SakLogger::init_test_console().unwrap();
    }
}
