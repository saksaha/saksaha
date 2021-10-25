use std::sync::Arc;
use log::{debug};

pub struct RPC {}

impl RPC {
    pub fn new(rpc_port: Option<u16>) -> RPC {
        return RPC {
        };
    }

    pub async fn start(&self) -> Result<u16, String> {
        debug!("Start rpc...");

        Ok(10000)
    }
}
