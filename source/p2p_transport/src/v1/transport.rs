use crypto::{SharedSecret, Secp256k1};
use p2p_identity::PeerId;
use tokio::net::TcpStream;
use std::sync::Arc;

pub struct Transport {
    // pub ip: String,
    // pub port: u16,
    pub stream: TcpStream,
    pub shared_secret: SharedSecret<Secp256k1>,
    pub peer_id: PeerId,
}

impl Transport {

}
