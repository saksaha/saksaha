use std::sync::{Arc,};

use crate::{common::SakResult, err_res};
use logger::log;
use tokio::sync::Mutex;

pub struct PeerStore {
    pub capacity: usize,
    pub curr_idx: Mutex<usize>,
    pub slots: Arc<Mutex<Vec<Arc<Mutex<Peer>>>>>,
}

impl PeerStore {
    pub fn new(capacity: usize) -> PeerStore {
        let mut slots = Vec::with_capacity(capacity);
        for i in 0..capacity {
            let peer = Peer::new(i);
            slots.push(Arc::new(Mutex::new(peer)));
        }

        PeerStore {
            curr_idx: Mutex::new(0),
            slots: Arc::new(Mutex::new(slots)),
            capacity,
        }
    }

    pub async fn next(&self) -> Option<Arc<Mutex<Peer>>> {
        let slots = &self.slots;
        let slots = slots.lock().await;
        let mut idx = self.curr_idx.lock().await;

        if let Some(p) = slots.get(*idx + 1) {
            *idx += 1;
            return Some(p.clone());
        } else {
            *idx = 0;
            match slots.get(*idx) {
                Some(p) => {
                    return Some(p.clone());
                },
                None => {
                    return None;
                }
            }
        }
    }

    // pub fn reserve_slot(&mut self) -> Option<usize> {
    //     let cap = self.capacity;

    //     for i in 0..cap {
    //         let idx = self.curr_idx + i % cap;
    //         let peer = match self.slots.get(idx) {
    //             Some(p) => p,
    //             None => {
    //                 return None;
    //             }
    //         };

    //         match peer.try_lock() {
    //             Ok(mut p) => {
    //                 if !p.reserved {
    //                     self.curr_idx = idx;
    //                     p.reserved = true;

    //                     log!(DEBUG, "Acquired a peer, at idx: {}\n", idx);

    //                     return Some(idx);
    //                 }
    //                 continue;
    //             }
    //             Err(_) => {
    //                 continue;
    //             }
    //         }
    //     }

    //     None
    // }
}

#[derive(Debug)]
pub struct Peer {
    pub i: usize,
    pub reserved: bool,
}

impl Peer {
    pub fn new(i: usize) -> Peer {
        Peer { i, reserved: false }
    }
}
