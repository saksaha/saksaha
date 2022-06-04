use sak_p2p_disc::Discovery;
use sak_p2p_ptable::PeerTable;
use std::sync::Arc;

pub(crate) struct P2PMonitor {
    // pub(crate) p2p_identity: Arc<P2PIdentity>,
    // pub(crate) p2p_port: u16,
    // pub(crate) identity: Arc<Identity>,
    pub(crate) peer_table: Arc<PeerTable>,
    // pub(crate) rpc_port: u16,
    pub(crate) p2p_discovery: Arc<Discovery>,
}
