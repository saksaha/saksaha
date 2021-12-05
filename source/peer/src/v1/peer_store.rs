use log::{debug, info, warn};
use p2p_identity::PeerId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, OwnedMutexGuard,
};

use super::peer::Peer;

const CAPACITY: usize = 50;

pub struct PeerStore {
    map: Mutex<HashMap<PeerId, Arc<Mutex<Peer>>>>,
    slots_tx: Sender<Arc<Mutex<Peer>>>,
    slots_rx: Mutex<Receiver<Arc<Mutex<Peer>>>>,
}

impl PeerStore {
    pub async fn init() -> Result<PeerStore, String> {
        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::channel::<Arc<Mutex<Peer>>>(CAPACITY);

            for _ in 0..CAPACITY {
                let empty_node = Arc::new(Mutex::new(Peer::new_empty()));

                match tx.send(empty_node).await {
                    Ok(_) => (),
                    Err(err) => {
                        return Err(format!(
                            "Can't fill an empty Peer to the slots, err: {}",
                            err
                        ));
                    }
                }
            }

            (tx, Mutex::new(rx))
        };

        let map = {
            let m = HashMap::with_capacity(CAPACITY);
            Mutex::new(m)
        };

        let ps = PeerStore {
            map,
            slots_tx,
            slots_rx,
        };

        Ok(ps)
    }

    pub async fn try_reserve(&self) -> Result<Arc<Mutex<Peer>>, String> {
        let mut slots_rx_guard = self.slots_rx.lock().await;

        match slots_rx_guard.try_recv() {
            Ok(p) => Ok(p),
            Err(err) => {
                Err(format!("No available slot to reserve, err: {}", err))
            }
        }
    }

    pub async fn reserve(&self) -> Result<Arc<Mutex<Peer>>, String> {
        let mut slots_rx_guard = self.slots_rx.lock().await;

        match slots_rx_guard.recv().await {
            Some(p) => Ok(p),
            None => {
                Err(format!("Slots channel might be closed"))
            }
        }
    }

    // pub async fn find(&self, peer_id: PeerId) -> Option<Arc<Mutex<Peer>>> {
    //     let map = self.map.lock().await;
    //     if let Some(p) = map.get(&peer_id) {
    //         Some(p.clone())
    //     } else {
    //         None
    //     }
    // }
}
