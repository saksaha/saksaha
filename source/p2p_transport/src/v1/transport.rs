use crate::Connection;
use crypto::{Secp256k1, SharedSecret};
use tokio::sync::RwLock;

pub struct Transport {
    pub conn: RwLock<Connection>,
    pub shared_secret: SharedSecret<Secp256k1>,
}

impl Transport {}
