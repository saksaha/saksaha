use crate::SakLogger;

#[test]
fn test_test_logger() {
    let test_dir = {
        let tempdir = std::env::temp_dir().join("sak_logger_test");

        std::fs::create_dir_all(&tempdir).unwrap();

        tempdir
    };

    SakLogger::init_for_test(&test_dir, &["test_1", "test_2"], "saksaha.log")
        .unwrap();

    tracing::info!(public_key = "power", "power 3131");
    tracing::info!(public_key2 = "power2", "power 3131");
    tracing::info!(public_key2 = 222, "power 3131");

    tracing::info!(target: "13113", "44444444444444444444444444444444");

    println!("2222");
}
