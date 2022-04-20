use p2p_identity::peer::UnknownPeer;

use crate::p2p::identity::Identity;

pub(crate) mod default;

#[derive(Debug)]
pub(crate) struct Config {
    pub(crate) rpc: RPCConfig,
    pub(crate) p2p: P2PConfig,
}

#[derive(Debug)]
pub(crate) struct P2PConfig {
    pub(crate) disc_dial_interval: Option<u16>,
    pub(crate) disc_table_capacity: Option<u16>,
    pub(crate) p2p_dial_interval: Option<u16>,
    pub(crate) bootstrap_urls: Option<Vec<String>>,
    pub(crate) disc_port: Option<u16>,
    pub(crate) p2p_port: Option<u16>,
    pub(crate) identity: Identity,
    pub(crate) unknown_peers: Vec<UnknownPeer>,
}

#[derive(Debug)]
pub(crate) struct RPCConfig {
    pub(crate) rpc_port: Option<u16>,
}
