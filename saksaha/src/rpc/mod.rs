use std::sync::Arc;
use log::{info, debug};
use tokio::net::TcpListener;

pub struct RPC {
    tcp_listener: TcpListener,
}

impl RPC {
    pub fn new(tcp_listener: TcpListener) -> RPC {
        RPC {
            tcp_listener,
        }
    }

    pub async fn start(&self) -> Result<u16, String> {
        info!("Start rpc...");

        Ok(10000)
    }
}
