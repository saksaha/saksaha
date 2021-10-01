use super::{Peer, PeerStatus};
use crate::{common::SakResult, err_res};
use logger::log;
use std::sync::Arc;
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
            let peer =
                Peer::new("".into(), "".into(), PeerStatus::NOT_INITIALZED);
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
                }
                None => {
                    return None;
                }
            }
        }
    }
}
