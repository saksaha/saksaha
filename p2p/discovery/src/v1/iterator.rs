use super::table::Node;
use std::sync::Arc;
use tokio::sync::{
    mpsc::{Receiver, Sender},
    Mutex,
};

pub struct Iterator {
    updates_tx: Arc<Sender<Arc<Node>>>,
    updates_rx: Arc<Mutex<Receiver<Arc<Node>>>>,
}

impl Iterator {
    pub fn new(
        updates_tx: Arc<Sender<Arc<Node>>>,
        updates_rx: Arc<Mutex<Receiver<Arc<Node>>>>,
    ) -> Iterator {
        Iterator {
            updates_tx,
            updates_rx,
        }
    }

    pub async fn next(&self) -> Result<Arc<Node>, String> {
        let mut updates_rx = self.updates_rx.lock().await;
        let node = match updates_rx.recv().await {
            Some(n) => n,
            None => {
                return Err(format!("Update channel is closed, fatal error"))
            }
        };

        Ok(node)
    }
}
