use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

pub(in crate::node) struct NodeRuntimeCtx {
    pub peer_table: Arc<PeerTable>,
}
