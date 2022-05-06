use crypto::{Secp256k1, SharedSecret};
use std::sync::Arc;
use tokio::net::TcpStream;

use crate::connection::Connection;

pub struct Transport<'a> {
    pub p2p_port: u16,
    pub public_key_str: String,
    pub conn: &'a mut Connection,
    pub shared_secret: SharedSecret<Secp256k1>,
}

impl<'a> Transport<'a> {}
