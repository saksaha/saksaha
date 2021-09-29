use crate::{common::SakResult, err_res};
use logger::log;
use tokio::sync::Mutex;

pub struct PeerStore {
    pub capacity: usize,
    pub curr_idx: usize,
    pub slots: Vec<Mutex<Peer>>,
}

impl PeerStore {
    pub fn new(capacity: usize) -> PeerStore {
        let mut slots = Vec::with_capacity(capacity);
        for i in 0..capacity {
            let p = Mutex::new(Peer::new(i));
            slots.push(p);
        }

        PeerStore {
            curr_idx: 0,
            slots,
            capacity,
        }
    }

    pub fn reserve_slot(&mut self) -> Option<usize> {
        let cap = self.capacity;

        for i in 0..cap {
            let idx = self.curr_idx + i % cap;
            let peer = match self.slots.get(idx) {
                Some(p) => p,
                None => {
                    return None;
                }
            };

            match peer.try_lock() {
                Ok(mut p) => {
                    if !p.reserved {
                        self.curr_idx = idx;
                        p.reserved = true;

                        log!(DEBUG, "Acquired a peer, at idx: {}\n", idx);

                        return Some(idx);
                    }
                    continue;
                }
                Err(_) => {
                    continue;
                }
            }
        }

        None
    }
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
