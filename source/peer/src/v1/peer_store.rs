use log::{debug, info, warn};
use p2p_identity::PeerId;
use std::{collections::{HashMap}, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, OwnedMutexGuard,
};

use crate::PeerValue;

use super::peer::Peer;

const CAPACITY: usize = 50;

pub struct PeerStore {
    pub map: Mutex<HashMap<PeerId, Arc<Peer>>>,
    slots_tx: Sender<Arc<Peer>>,
    slots_rx: Mutex<Receiver<Arc<Peer>>>,

}

impl PeerStore {
    pub async fn init() -> Result<PeerStore, String> {
        let (slots_tx, slots_rx) = {
            let (tx, rx) = mpsc::channel::<Arc<Peer>>(CAPACITY);

            for _ in 0..CAPACITY {
                let empty_node = Arc::new(Peer::new_empty());

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

    pub async fn reserve(&self) -> Result<Arc<Peer>, String> {
        let mut slots_rx_guard = self.slots_rx.lock().await;

        match slots_rx_guard.recv().await {
            Some(p) => Ok(p),
            None => {
                Err(format!("Slots channel might be closed"))
            }
        }
    }

    pub async fn register(&self, peer: Arc<Peer>) {
        let mut map = self.map.lock().await;

        let peer_val = peer.value.lock().await;
        if let PeerValue::Registered(p) = &*peer_val {
            let peer_id = p.transport.peer_id;
            map.insert(peer_id, peer.clone());

            debug!("Peer store added peer_id: {:?}", peer_id);
        } else {
        }
    }

    // pub async fn DEMO__add(&self, peer: Arc<Peer>) {
    //     let list = self._list.lock().await;
    //     let found = false;
    //     for peer in list.iter() {
    //         let p_value = peer.value.lock().await;
    //         // p_value.
    //     }
    // }
}
