use log::{debug, info};
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct RPC {
    tcp_listener: Arc<TcpListener>,
}

impl RPC {
    pub fn new(tcp_listener: Arc<TcpListener>) -> RPC {
        RPC { tcp_listener }
    }

    pub async fn start(&self) -> Result<u16, String> {
        info!("Start rpc...");

        Ok(10000)
    }
}
