use p2p_identity::addr::UnknownAddr;

pub(crate) mod local_1;

pub(crate) struct DefaultConfig {
    pub(crate) p2p: DefaultP2PConfig,
}

pub(crate) struct DefaultP2PConfig {
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl DefaultConfig {
    pub(crate) fn new_empty() -> DefaultConfig {
        DefaultConfig {
            p2p: DefaultP2PConfig {
                bootstrap_addrs: vec![],
            },
        }
    }
}
