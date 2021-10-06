use super::{Peer, Status};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub struct Filter;

impl Filter {
    pub fn not_initialized(peer: MutexGuard<Peer>) -> bool {
        peer.status == Status::NotInitialized
    }

    pub fn discovery_success(peer: MutexGuard<Peer>) -> bool {
        peer.status == Status::DiscoverySuccess
    }
}

pub struct PeerStore {
    pub capacity: usize,
    pub curr_idx: Mutex<usize>,
    pub slots: Arc<Mutex<Vec<Arc<Mutex<Peer>>>>>,
}

impl PeerStore {
    pub fn new(
        capacity: usize,
        bootstrap_urls: Option<Vec<String>>,
    ) -> PeerStore {
        let mut slots = Vec::with_capacity(capacity);

        if let Some(urls) = bootstrap_urls {
            for u in urls {
                let p = match Peer::parse(u.to_owned()) {
                    Ok(p) => Arc::new(Mutex::new(p)),
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Cannot parse url, url: {}, err: {}\n",
                            u,
                            err
                        );
                        continue;
                    }
                };

                slots.push(p);
            }
        }

        PeerStore {
            curr_idx: Mutex::new(0),
            slots: Arc::new(Mutex::new(slots)),
            capacity,
        }
    }

    pub async fn next(
        &self,
        filter: &(dyn Fn(MutexGuard<Peer>) -> bool + Sync + Send),
    ) -> Option<Arc<Mutex<Peer>>> {
        let slots = &self.slots;
        let slots = slots.lock().await;
        let capacity = self.capacity;

        let mut curr_idx = self.curr_idx.lock().await;
        let start_idx = *curr_idx + 1;

        for i in start_idx..start_idx + capacity {
            let idx = i % capacity;

            if let Some(p) = slots.get(idx) {
                let peer_lock = match p.try_lock() {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                if filter(peer_lock) {
                    *curr_idx = idx;

                    return Some(p.clone());
                } else {
                    continue;
                }
            }
        }

        *curr_idx = 0;
        None
    }
}
