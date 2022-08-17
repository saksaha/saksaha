use crate::UpgradedConn;
use tokio::sync::RwLock;

pub struct Transport {
    pub conn: RwLock<UpgradedConn>,
}
