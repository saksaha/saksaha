use p2p_identity::addr::UnknownAddr;

pub(crate) mod local_1;

pub(crate) struct DevConfig {
    pub(crate) p2p: DevP2PConfig,
}

pub(crate) struct DevP2PConfig {
    pub(crate) bootstrap_addrs: Vec<UnknownAddr>,
}

impl DevConfig {
    pub(crate) fn new_empty() -> DevConfig {
        DevConfig {
            p2p: DevP2PConfig {
                bootstrap_addrs: vec![],
            },
        }
    }
}
