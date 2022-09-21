use crate::SakLogger;

#[test]
fn test_test_logger() {
    let test_dir = {
        let tempdir = std::env::temp_dir().join("sak_logger_test");
        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    println!("33333333333");

    let _logger =
        SakLogger::init_test_persisted(&test_dir, &["test_1", "test_2"], "saksaha22.log").unwrap();

    tracing::debug!(public_key = "test_1", "test 1 log");
    tracing::debug!(public_key = "test_3", "invalid");
    tracing::debug!(public_key = "test_2", "test 2 log");

    tracing::debug!("22222222222222222222222");

    println!("2222");
}
