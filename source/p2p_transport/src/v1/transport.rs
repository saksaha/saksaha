use crate::connection::Connection;
use crypto::{Secp256k1, SharedSecret};
use p2p_active_calls::CallGuard;
use p2p_discovery::AddrGuard;
use std::sync::Arc;
use tokio::net::TcpStream;

pub struct Transport {
    pub call_guard: CallGuard,
    pub p2p_port: u16,
    pub public_key_str: String,
    pub conn: Connection,
    pub shared_secret: SharedSecret<Secp256k1>,
    pub addr_guard: Option<AddrGuard>,
}

impl Transport {}
