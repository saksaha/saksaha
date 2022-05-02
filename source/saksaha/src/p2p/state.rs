use p2p_active_calls::ActiveCalls;
use p2p_identity::identity::P2PIdentity;
use peer::PeerStore;
use std::sync::Arc;
use task_queue::TaskQueue;

pub(crate) struct HostState {
    pub(crate) p2p_identity: Arc<P2PIdentity>,
    pub(crate) my_rpc_port: u16,
    pub(crate) my_p2p_port: u16,
    pub(crate) peer_store: Arc<PeerStore>,
}
