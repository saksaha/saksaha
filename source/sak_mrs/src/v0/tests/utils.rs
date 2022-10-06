use sak_kv_db::{Options, DB};
use sak_logger::{info, SakLogger};

pub(crate) struct MRSTestUtils;

impl MRSTestUtils {
    pub fn init_test(app_prefixes: Vec<&str>) {
        SakLogger::init_test_console().unwrap();
    }
}
