use crate::{Peer, PeerTableError};
use std::sync::Arc;
use tokio::sync::{mpsc::UnboundedReceiver, RwLock};

pub struct PeerIterator {
    pub(super) peers_rx: UnboundedReceiver<Arc<Peer>>,
}

impl PeerIterator {
    pub async fn next(&mut self) -> Result<Arc<Peer>, PeerTableError> {
        let peer = self.peers_rx.recv().await;

        match peer {
            Some(p) => Ok(p),
            None => {
                return Err(format!("Peer rx has been closed, fatal error").into());
            }
        }
    }
}
