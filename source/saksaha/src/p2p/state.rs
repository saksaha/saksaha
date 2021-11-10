use super::task::Task;
use p2p_identity::Identity;
use peer::PeerStore;
use task::task_queue::{self, TaskQueue};
use std::sync::Arc;

pub(crate) struct HostState {
    pub identity: Arc<Identity>,
    pub task_queue: Arc<TaskQueue<Task>>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub peer_store: Arc<PeerStore>,
}

impl HostState {
    pub fn new(
        identity: Arc<Identity>,
        my_rpc_port: u16,
        my_p2p_port: u16,
        task_queue: Arc<TaskQueue<Task>>,
        peer_store: Arc<PeerStore>,
    ) -> HostState {
        HostState {
            identity,
            task_queue,
            my_p2p_port,
            my_rpc_port,
            peer_store,
        }
    }
}
