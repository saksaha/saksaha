use log::{debug, info, warn};
use saksaha_p2p_identity::PeerId;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, OwnedMutexGuard,
};

use super::peer::Peer;

const CAPACITY: usize = 50;

// pub struct Filter;

// impl Filter {
//     pub fn not_initialized(peer: &OwnedMutexGuard<Peer>) -> bool {
//         return peer.status == Status::NotInitialized;
//     }

//     pub fn discovery_success(peer: &OwnedMutexGuard<Peer>) -> bool {
//         return peer.status == Status::DiscoverySuccess;
//     }
// }

pub struct PeerStore {
    // pub slots: Arc<Mutex<Vec<Arc<Mutex<Peer>>>>>,
    map: Mutex<HashMap<PeerId, Peer>>,
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

    // pub async fn next(
    //     &self,
    //     last_idx: Option<usize>,
    //     filter: &(dyn Fn(&OwnedMutexGuard<Peer>) -> bool + Sync + Send),
    // ) -> Option<(OwnedMutexGuard<Peer>, usize)> {
    //     let slots = self.slots.lock().await;

    //     let start_idx = match last_idx {
    //         Some(i) => i + 1,
    //         None => 0,
    //     };

    //     let cap = self.capacity;

    //     for i in start_idx..start_idx + cap {
    //         let idx = i % cap;

    //         let peer = match slots.get(idx) {
    //             Some(p) => p.to_owned(),
    //             None => {
    //                 warn!(
    //                     "There is an empty slot. Something might be wrong"
    //                 );
    //                 return None;
    //             }
    //         };

    //         let peer_guard = match peer.try_lock_owned() {
    //             Ok(p) => p,
    //             Err(_) => continue,
    //         };

    //         if filter(&peer_guard) {
    //             return Some((peer_guard, idx));
    //         } else {
    //             continue;
    //         }
    //     }

    //     None
    // }

    // pub async fn find(
    //     &self,
    //     filter: &(dyn Fn(&OwnedMutexGuard<Peer>) -> bool + Sync + Send),
    // ) -> Option<(OwnedMutexGuard<Peer>, usize)> {
    //     let slots = self.slots.lock().await.to_owned();

    //     for (idx, p) in slots.into_iter().enumerate() {
    //         let peer_guard = match p.try_lock_owned() {
    //             Ok(p) => p,
    //             Err(_) => continue,
    //         };

    //         if filter(&peer_guard) {
    //             return Some((peer_guard, idx));
    //         } else {
    //             continue;
    //         }
    //     }
    //     None
    // }

    // pub async fn reserve(&self) -> Option<(OwnedMutexGuard<Peer>, usize)> {
    //     self.find(&|peer| peer.status == Status::Empty).await
    // }
}
