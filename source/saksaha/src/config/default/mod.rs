use p2p_identity::peer::UnknownPeer;

use super::Identity;

pub(crate) mod dev_local;

pub(crate) struct DConfig {
    pub(crate) p2p: DefaultP2PConfig,
}

pub(crate) struct DefaultP2PConfig {
    pub(crate) unknown_peers: Vec<UnknownPeer>,
}

pub(crate) fn get_empty_default_config() -> DConfig {
    DConfig {
        p2p: DefaultP2PConfig {
            unknown_peers: vec![],
        },
    }
}
