use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::{self, TaskQueue};
use std::sync::Arc;
use super::task::Task;

pub struct HostState {
    pub identity: Arc<Identity>,
}

impl HostState {
    pub fn new(
        identity: Arc<Identity>,
        my_rpc_port: u16,
        my_p2p_port: u16,
        task_queue: Arc<TaskQueue<Task>>,
    ) -> HostState {
        HostState { identity }
    }
}
