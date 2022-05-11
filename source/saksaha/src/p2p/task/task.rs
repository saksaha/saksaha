use crate::p2p::state::HostState;
use p2p_discovery::AddrGuard;
use std::sync::Arc;

pub(crate) enum P2PTask {
    InitiateHandshake {
        addr_guard: AddrGuard,
        host_state: Arc<HostState>,
    },
}

impl std::fmt::Display for P2PTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitiateHandshake { addr_guard, .. } => {
                let known_addr = addr_guard.get_known_addr();
                write!(
                    f,
                    "InitiateHandshake, dest: {}, at: {}",
                    known_addr.p2p_endpoint(),
                    known_addr.known_at,
                )
            }
        }
    }
}
