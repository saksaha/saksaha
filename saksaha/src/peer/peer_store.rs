use super::{Peer, Status};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, OwnedMutexGuard};

pub struct Filter;

impl Filter {
    pub fn not_initialized(peer: &OwnedMutexGuard<Peer>) -> bool {
        return peer.status == Status::NotInitialized;
    }

    pub fn discovery_success(peer: &OwnedMutexGuard<Peer>) -> bool {
        return peer.status == Status::DiscoverySuccess;
    }
}

pub struct PeerStore {
    pub slots: Arc<Mutex<Vec<Arc<Mutex<Peer>>>>>,
    // slots: Vec<Mutex<Peer>>,
    pub capacity: usize,
}

impl PeerStore {
    pub fn new(
        capacity: usize,
        // bootstrap_urls: Option<Vec<String>>,
    ) -> Result<PeerStore, String> {
        // let mut slots = Vec::with_capacity(capacity);
        let slots = Arc::new(Mutex::new(Vec::with_capacity(capacity)));
        let mut slots_guard = match slots.try_lock() {
            Ok(s) => s,
            Err(err) => {
                return Err(format!("Cannot acquire slots, err: {}\n", err))
            }
        };

        // let bootstrap_urls = match bootstrap_urls {
        //     Some(u) => u,
        //     None => vec![],
        // };

        // let default_urls = crate::default_bootstrap_urls!()
        //     .into_iter()
        //     .map(|url| url.to_string())
        //     .collect::<Vec<String>>();

        // let urls_combined = [bootstrap_urls, default_urls].concat();
        let mut count = 0;

        log!(
            DEBUG,
            "*****************************************************\n"
        );
        log!(DEBUG, "* Peer store\n");
        // for u in urls_combined {
        //     let p = match Peer::parse(u.to_owned()) {
        //         Ok(p) => {
        //             log!(
        //                 DEBUG,
        //                 "* [{}], peer_id: {}, ip: {}, disc_port: {}\n",
        //                 count,
        //                 p.peer_id,
        //                 p.ip,
        //                 p.disc_port
        //             );
        //             Arc::new(Mutex::new(p))
        //             // Mutex::new(p)
        //         }
        //         Err(err) => {
        //             log!(DEBUG, "Cannot parse url, url: {}, err: {}\n", u, err);
        //             continue;
        //         }
        //     };

        //     slots_guard.push(p);
        //     count += 1;
        // }

        for _ in 0..capacity {
            let p = Peer::new_empty();
            slots_guard.push(Arc::new(Mutex::new(p)));
        }

        log!(
            DEBUG,
            "* Peer store init result, count: {}, len: {}, capacity: {}\n",
            count,
            slots_guard.len(),
            capacity
        );
        log!(
            DEBUG,
            "*****************************************************\n"
        );

        drop(slots_guard);
        let ps = PeerStore { slots, capacity };

        Ok(ps)
    }

    pub async fn next(
        &self,
        last_idx: Option<usize>,
        filter: &(dyn Fn(&OwnedMutexGuard<Peer>) -> bool + Sync + Send),
    ) -> Option<(OwnedMutexGuard<Peer>, usize)> {
        let slots = self.slots.lock().await;

        let start_idx = match last_idx {
            Some(i) => i + 1,
            None => 0,
        };

        let cap = self.capacity;

        for i in start_idx..start_idx + cap {
            let idx = i % cap;

            let peer = match slots.get(idx) {
                Some(p) => p.to_owned(),
                None => {
                    log!(
                        DEBUG,
                        "There is an empty slot. Something might be wrong\n"
                    );
                    return None;
                }
            };

            let peer_guard = match peer.try_lock_owned() {
                Ok(p) => p,
                Err(_) => continue,
            };

            if filter(&peer_guard) {
                return Some((peer_guard, idx));
            } else {
                continue;
            }
        }

        None
    }

    pub async fn find(
        &self,
        filter: &(dyn Fn(&OwnedMutexGuard<Peer>) -> bool + Sync + Send),
    ) -> Option<(OwnedMutexGuard<Peer>, usize)> {
        let slots = self.slots.lock().await.to_owned();

        for (idx, p) in slots.into_iter().enumerate() {
            let peer_guard = match p.try_lock_owned() {
                Ok(p) => p,
                Err(_) => continue,
            };

            if filter(&peer_guard) {
                return Some((peer_guard, idx));
            } else {
                continue;
            }
        }
        None
    }

    pub async fn reserve(&self) -> Option<(OwnedMutexGuard<Peer>, usize)> {
        self.find(&|peer| peer.status == Status::Empty).await
    }
}
