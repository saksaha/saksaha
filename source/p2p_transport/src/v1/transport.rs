use crate::connection::Connection;
use crypto::{Secp256k1, SharedSecret};
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct Transport {
    pub p2p_port: u16,
    pub public_key_str: String,
    pub conn: Connection,
    pub shared_secret: SharedSecret<Secp256k1>,
}

impl Transport {}
