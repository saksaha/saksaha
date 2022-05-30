use p2p_discovery::Addr;
use p2p_identity::Identity;
use p2p_peer_table::PeerTable;
use std::sync::Arc;
use tokio::sync::RwLock;

pub(crate) enum P2PTask {
    InitiateHandshake {
        // addr_guard: AddrGuard,
        addr: Arc<RwLock<Addr>>,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
    },
}

impl std::fmt::Display for P2PTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::InitiateHandshake { addr_lock, .. } => {
            Self::InitiateHandshake { addr, .. } => {
                write!(f, "InitiateHandshake",)
            }
        }
    }
}
