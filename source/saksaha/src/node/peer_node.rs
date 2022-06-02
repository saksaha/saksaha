use p2p_peer_table::Peer;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) struct PeerNode {
    pub(crate) peer: Arc<Peer>,
}

impl PeerNode {}
