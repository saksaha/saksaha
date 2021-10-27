use std::sync::Arc;
use log::{info, debug};

pub struct RPC {}

impl RPC {
    pub fn new(rpc_port: Option<u16>) -> RPC {
        return RPC {
        };
    }

    pub async fn start(&self) -> Result<u16, String> {
        info!("Start rpc...");

        Ok(10000)
    }
}
