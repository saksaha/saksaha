use std::{sync::Arc, time::SystemTime};

use tokio::sync::Mutex;

use crate::p2p::peer::peer_store::PeerStore;

pub struct Handshake {
    peer_store: Arc<PeerStore>,
    is_running: Arc<Mutex<bool>>,
}

impl Handshake {
    pub fn new(peer_store: Arc<PeerStore>) -> Handshake {
        Handshake {
            is_running: Arc::new(Mutex::new(false)),
            peer_store,
        }
    }

    pub fn run(&self) {
        let is_running = self.is_running.clone();
        let peer_store = self.peer_store.clone();

        tokio::spawn(async move {
            let mut is_running_lock = is_running.lock().await;
            *is_running_lock = true;
            std::mem::drop (is_running_lock);

            loop {
                let start = SystemTime::now();

                // &self.peer_store.next();
            }

        });
    }

    pub async fn wakeup(&self) {}
}
