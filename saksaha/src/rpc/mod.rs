use std::sync::Arc;

use logger::log;

use crate::node::task_manager::TaskManager;

pub struct RPC {
    task_mng: Arc<TaskManager>,
}

impl RPC {
    pub fn new(task_mng: Arc<TaskManager>, rpc_port: u16) -> RPC {
        return RPC {
            task_mng,
        };
    }

    pub async fn start(&self) {
        log!(DEBUG, "Start rpc...\n");
    }
}
