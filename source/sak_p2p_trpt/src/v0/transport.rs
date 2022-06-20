use crate::UpgradedConnection;
use tokio::sync::RwLock;

pub struct Transport {
    pub conn: RwLock<UpgradedConnection>,
}
