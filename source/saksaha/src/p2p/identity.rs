use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Identity {
    pub secret: String,
    pub public_key: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Peer {
    pub disc_port: u16,
    pub p2p_port: u16,
    pub secret: String,
    pub public_key: String,
}
