use std::time::Duration;

const RPC_PORT: u16 = 37701;

pub(crate) struct RPC {
    rpc_port: u16,
}

impl RPC {
    pub fn init(rpc_port: Option<u16>) -> RPC {
        let rpc_port = rpc_port.unwrap_or(RPC_PORT);

        RPC { rpc_port }
    }

    pub async fn run(&self) {
        println!("rpc starts");

        tokio::time::sleep(Duration::from_secs(100)).await;
    }
}
