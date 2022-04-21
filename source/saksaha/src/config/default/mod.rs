use p2p_identity::addr::Addr;

use super::Identity;

pub(crate) mod dev_local;

pub(crate) struct DefaultConfig {
    pub(crate) p2p: DefaultP2PConfig,
}

pub(crate) struct DefaultP2PConfig {
    pub(crate) bootstrap_addrs: Vec<Addr>,
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
