pub mod status;

pub use status::Status;
use std::sync::Arc;
use logger::log;
use crate::{common::Error };

pub struct RPC {
}

impl RPC {
    pub fn new(rpc_port: Option<u16>) -> RPC {
        return RPC {
        };
    }

    pub async fn start(&self) -> Status<u16, Error> {
        log!(DEBUG, "Start rpc...\n");

        Status::Launched(10000)
    }
}
