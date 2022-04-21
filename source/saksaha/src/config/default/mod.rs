use p2p_identity::peer::UnknownPeer;

use super::Identity;

pub(crate) mod dev_local;

pub(crate) struct DefaultConfig {
    pub(crate) p2p: DefaultP2PConfig,
}

pub(crate) struct DefaultP2PConfig {
    pub(crate) unknown_peers: Vec<UnknownPeer>,
}

impl DefaultConfig {
    pub(crate) fn new_empty() -> DefaultConfig {
        DefaultConfig {
            p2p: DefaultP2PConfig {
                unknown_peers: vec![],
            },
        }
    }
}
