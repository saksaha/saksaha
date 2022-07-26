use sak_p2p_discovery::DiscAddr;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use std::sync::Arc;

pub(crate) enum P2PTask {
    InitiateHandshake {
        addr: Arc<DiscAddr>,
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
                    addr.known_addr.get_p2p_endpoint(),
                )
            }
        }
    }
}
