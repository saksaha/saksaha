use p2p_active_calls::ActiveCalls;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::PeerTable;
use std::sync::Arc;

pub(crate) struct HostState {
    pub(crate) p2p_identity: Arc<P2PIdentity>,
    pub(crate) p2p_active_calls: Arc<ActiveCalls>,
    pub(crate) p2p_port: u16,
    pub(crate) p2p_peer_table: Arc<PeerTable>,
    pub(crate) rpc_port: u16,
}
