use sak_logger::SakLogger;

pub(crate) struct VMTestUtils;

impl VMTestUtils {
    pub fn init_test(app_prefixes: Vec<&str>) {
        SakLogger::init_test_console().unwrap();
    }
}
