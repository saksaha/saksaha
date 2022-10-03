use sak_logger::{self, SakLogger};

#[tokio::test(flavor = "multi_thread")]
async fn test_gen_pi_with_32_depth() {
    SakLogger::init_test_console().unwrap();

    let s = sak_proof::pi_gen_1_depth_32();

    println!("pi: {:?}", s);
}
