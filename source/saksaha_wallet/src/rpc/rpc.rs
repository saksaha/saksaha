use std::time::Duration;

pub(crate) struct RPC {}

impl RPC {
    pub fn init() -> RPC {
        RPC {}
    }

    pub async fn run(&self) {
        println!("rpc starts");

        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}
