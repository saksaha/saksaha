use super::*;

#[cfg(test)]
mod test {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_blockchain_foo() {
        init();

        println!("a;weopfa");
        // let ledger_db;
        // let blockchain;
    }
}
