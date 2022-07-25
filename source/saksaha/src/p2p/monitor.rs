use sak_p2p_discovery::Discovery;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

pub(crate) struct P2PMonitor {
    pub(crate) peer_table: Arc<PeerTable>,
    pub(crate) p2p_discovery: Arc<Discovery>,
}
