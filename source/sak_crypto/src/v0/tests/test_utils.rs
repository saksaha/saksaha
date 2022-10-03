use sak_logger::SakLogger;

pub fn init_test() {
    SakLogger::init_test_console().unwrap();
}
