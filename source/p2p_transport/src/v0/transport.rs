use crate::Connection;
use crypto::SharedSecret;
use tokio::sync::RwLock;

pub struct Transport {
    pub conn: RwLock<Connection>,
    pub shared_secret: SharedSecret,
}

impl Transport {}
