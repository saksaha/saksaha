use super::task::Task;
use p2p_active_calls::ActiveCalls;
use p2p_identity::P2PIdentity;
use peer::PeerStore;
use std::sync::Arc;
use task::task_queue::{self, TaskQueue};

pub(crate) struct HostState {
    pub identity: Arc<P2PIdentity>,
    pub task_queue: Arc<TaskQueue<Task>>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub peer_store: Arc<PeerStore>,
    pub handshake_active_calls: Arc<ActiveCalls>,
}

impl HostState {
    pub fn new(
        identity: Arc<P2PIdentity>,
        my_rpc_port: u16,
        my_p2p_port: u16,
        task_queue: Arc<TaskQueue<Task>>,
        peer_store: Arc<PeerStore>,
        handshake_active_calls: Arc<ActiveCalls>,
    ) -> HostState {
        HostState {
            identity,
            task_queue,
            my_p2p_port,
            my_rpc_port,
            peer_store,
            handshake_active_calls,
        }
    }
}
