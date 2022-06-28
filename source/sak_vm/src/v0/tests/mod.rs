#[cfg(test)]
mod test {

    use sak_contract_std::{Request, Storage};

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_validator_ctrt_init_fn() {
        assert_eq!("init", "init");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_validator_ctrt_query_fn() {
        assert_eq!("query", "query");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_call_validator_ctrt_execute_fn() {
        assert_eq!("exeucte", "exeucte");
    }
}
