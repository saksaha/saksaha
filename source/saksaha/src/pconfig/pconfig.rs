use p2p_identity::addr::UnknownAddr;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PConfig {
    pub p2p: PersistedP2PConfig,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedP2PConfig {
    pub secret: String,
    pub public_key: String,
    pub bootstrap_addrs: Option<Vec<UnknownAddr>>,
    pub p2p_port: Option<u16>,
    pub disc_port: Option<u16>,
}
