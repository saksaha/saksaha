use sak_logger::SakLogger;

pub(crate) struct ContractTestUtils;

impl ContractTestUtils {
    pub fn init_test() {
        SakLogger::init_test_console().unwrap();
    }
}
