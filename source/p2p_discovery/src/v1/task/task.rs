use p2p_addr::UnknownAddr;

pub(crate) enum DiscoveryTask {
    InitiateWhoAreYou { addr: UnknownAddr },
}

impl std::fmt::Display for DiscoveryTask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitiateWhoAreYou { addr } => {
                write!(f, "InitiateWhoAreYou [dest: {}]", addr.disc_endpoint())
            }
        }
    }
}
