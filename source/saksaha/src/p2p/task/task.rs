use p2p_discovery::Addr;
use p2p_identity::Identity;
use p2p_peer_table::PeerTable;
use std::sync::Arc;

pub(crate) enum P2PTask {
    InitiateHandshake {
        addr: Arc<Addr>,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
    },
}

impl std::fmt::Display for P2PTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitiateHandshake { addr, .. } => {
                write!(
                    f,
                    "InitiateHandshake, p2p_endpointt: {}",
                    addr.known_addr.p2p_endpoint(),
                )
            }
        }
    }
}
