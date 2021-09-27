use crate::{common::SakResult, err_res};
use logger::log;
use tokio::sync::Mutex;
use std::{
    convert::TryInto,
    future::Future,
    // sync::{Mutex},
};

pub struct PeerStore {
    pub capacity: usize,
    pub curr_idx: usize,
    pub slots: Mutex<Vec<Mutex<Peer>>>,
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
            slots: Mutex::new(slots),
            capacity,
        }
    }

    pub async fn f() {
        // return slots
    }

    pub async fn take_empty_slot<F>(&self, callback: F) -> SakResult<bool>
    where
        F: for<'b> Fn(&'b mut Peer) -> futures::future::BoxFuture<'b, ()>,
        // F: Fn() -> () + Send + 'static,
        // Fut: Future<Output = bool>,
        // C: Fn(&'a Peer) -> Peer
    {
        println!("333");
        let cap = self.capacity;

        // for i in 0..cap {
        //     let idx = self.curr_idx + i % cap;
        //     let peer = match self.slots.get(idx) {
        //         Some(p) => p,
        //         None => {
        //             return err_res!(
        //                 "Error getting peer in the slot, \
        //                 it may have been accidentally removed"
        //             );
        //         }
        //     };

        //     let mut peer = match peer.try_lock() {
        //         Ok(p) => {
        //             log!(DEBUG, "Acquired a peer, at idx: {}\n", idx);
        //             p
        //         }
        //         Err(_) => {
        //             continue;
        //         }
        //     };

        //     callback(&mut peer).await;

        //     println!("44: {}", peer.i);
        //     return Ok(true);
        // }

        Ok(false)
    }
}

#[derive(Debug)]
pub struct Peer {
    pub i: usize,
}

impl Peer {
    pub fn new(i: usize) -> Peer {
        Peer { i }
    }
}
