use crate::p2p::identity::Identity;
use p2p_identity::addr::Addr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub identity: Identity,
    pub bootstrap_addrs: Option<Vec<Addr>>,
    pub p2p_port: Option<u16>,
    pub disc_port: Option<u16>,
}
