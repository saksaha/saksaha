use sak_logger::SakLogger;

pub(crate) struct StoreAccessorTestUtils;

impl StoreAccessorTestUtils {
    pub fn init_test(app_prefixes: Vec<&str>) {
        SakLogger::init_test_console().unwrap();
    }
}
