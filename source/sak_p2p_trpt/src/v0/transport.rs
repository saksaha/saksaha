use crate::Connection;
use sak_crypto::SharedSecret;
use tokio::sync::RwLock;

pub struct Transport {
    pub conn: RwLock<Connection>,
    pub shared_secret: SharedSecret,
}

impl Transport {}
