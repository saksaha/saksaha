use crate::SakLogger;

#[test]
fn test_test_logger() {
    let test_dir = {
        let tempdir = std::env::temp_dir().join("sak_logger_test");

        std::fs::create_dir_all(&tempdir).unwrap();

        tempdir
    };

    let log_dirs = &[test_dir.join("test_1"), test_dir.join("test_2")];

    SakLogger::init_for_test(log_dirs, "saksaha.log").unwrap();

    tracing::info!(a_field = 33333333, "power 3131");

    tracing::info!(target: "13113", "44444444444444444444444444444444");

    println!("2222");
}
