use p2p_peer_table::Peer;
use std::sync::Arc;

pub(crate) struct PeerNode {
    pub(crate) peer: Arc<Peer>,
}

impl PeerNode {}
