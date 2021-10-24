use std::sync::Arc;
use logger::log;

pub struct RPC {}

impl RPC {
    pub fn new(rpc_port: Option<u16>) -> RPC {
        return RPC {
        };
    }

    pub async fn start(&self) -> Result<u16, String> {
        log!(DEBUG, "Start rpc...\n");

        Ok(10000)
    }
}
