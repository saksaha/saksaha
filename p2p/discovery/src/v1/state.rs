use std::sync::Arc;
use saksaha_p2p_active_calls::ActiveCalls;
use saksaha_p2p_identity::Identity;
use saksaha_task::task_queue::TaskQueue;
use crate::task::Task;

use super::{table::Table};

pub(crate) struct DiscState {
    pub identity: Arc<Identity>,
    pub my_disc_port: u16,
    pub my_p2p_port: u16,
    pub table: Arc<Table>,
    pub task_queue: Arc<TaskQueue<Task>>,
    pub _active_calls: Arc<ActiveCalls>,
}

impl DiscState {
    pub fn new(
        identity: Arc<Identity>,
        table: Arc<Table>,
        active_calls: Arc<ActiveCalls>,
        my_disc_port: u16,
        my_p2p_port: u16,
        task_queue: Arc<TaskQueue<Task>>,
    ) -> DiscState {
        DiscState {
            identity,
            my_disc_port,
            my_p2p_port,
            table,
            task_queue,
            _active_calls: active_calls,
        }
    }
}
