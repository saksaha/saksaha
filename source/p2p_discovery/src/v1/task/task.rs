use crate::state::DiscState;
use p2p_identity::addr::UnknownAddr;
use std::sync::Arc;

pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou {
        addr: UnknownAddr,
        disc_state: Arc<DiscState>,
    },
}

impl std::fmt::Display for DiscoveryTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitiateWhoAreYou { addr, .. } => {
                write!(f, "InitiateWhoAreYou, addr: {:?}", addr)
            }
        }
    }
}
