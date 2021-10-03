pub mod status;

pub use status::Status;
use std::sync::Arc;
use logger::log;
use crate::{common::Error, node::task_manager::TaskManager};

pub struct RPC {
    task_mng: Arc<TaskManager>,
}

impl RPC {
    pub fn new(task_mng: Arc<TaskManager>, rpc_port: u16) -> RPC {
        return RPC {
            task_mng,
        };
    }

    pub async fn start(&self) -> Status<u16, Error> {
        log!(DEBUG, "Start rpc...\n");

        Status::Launched(10000)
    }
}
