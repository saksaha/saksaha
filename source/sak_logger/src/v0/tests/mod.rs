use crate::SakLogger;

#[test]
fn test_test_logger() {
    let test_dir = {
        let tempdir = std::env::temp_dir().join("sak_logger_test");
        std::fs::create_dir_all(&tempdir).unwrap();
        tempdir
    };

    let pk_1 = "pk_1";
    let pk_2 = "pk_2";
    let pk_3 = "pk_3";

    SakLogger::init_test_persisted(test_dir).unwrap();

    tracing::debug!(public_key = pk_1, "test 1 log");
    tracing::error!(public_key = pk_1, "test 1 error");
    tracing::info!(public_key = pk_1, "test 1 info");

    tracing::debug!(public_key = pk_2, "invalid");
    tracing::debug!(public_key = pk_3, "test 2 log");

    tracing::debug!("22222222222222222222222");
}
