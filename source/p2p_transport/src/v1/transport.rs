use crypto::{Secp256k1, SharedSecret};
use p2p_identity::PeerId;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct Transport {
    // pub ip: String,
    // pub port: u16,
    pub stream: TcpStream,
    pub shared_secret: SharedSecret<Secp256k1>,
    pub peer_id: PeerId,
}

impl Transport {}
