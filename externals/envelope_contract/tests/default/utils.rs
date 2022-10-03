use sak_logger::SakLogger;

pub fn init_test_log() {
    SakLogger::init_test().unwrap();
}
