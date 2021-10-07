use super::{Peer, Status};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub struct Filter;

impl Filter {
    pub fn not_initialized(peer: &MutexGuard<Peer>) -> bool {
        return peer.status == Status::NotInitialized;
    }

    pub fn discovery_success(peer: &MutexGuard<Peer>) -> bool {
        return peer.status == Status::DiscoverySuccess;
    }
}

pub struct PeerStore {
    slots: Vec<Arc<Mutex<Peer>>>,
    pub capacity: usize,
}

impl PeerStore {
    pub fn new(
        capacity: usize,
        bootstrap_urls: Option<Vec<String>>,
    ) -> PeerStore {
        let mut slots = Vec::with_capacity(capacity);

        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => vec![],
        };

        let default_urls = crate::default_bootstrap_urls!()
            .into_iter()
            .map(|url| url.to_string())
            .collect::<Vec<String>>();

        let urls_combined = [bootstrap_urls, default_urls].concat();
        let mut count = 0;

        log!(
            DEBUG,
            "*****************************************************\n"
        );
        log!(DEBUG, "* Peer store\n");
        for u in urls_combined {
            let p = match Peer::parse(u.to_owned()) {
                Ok(p) => {
                    log!(
                        DEBUG,
                        "* [{}], peer_id: {}, ip: {}, disc_port: {}\n",
                        count,
                        p.peer_id,
                        p.ip,
                        p.disc_port
                    );
                    Arc::new(Mutex::new(p))
                }
                Err(err) => {
                    log!(DEBUG, "Cannot parse url, url: {}, err: {}\n", u, err);
                    continue;
                }
            };

            slots.push(p);
            count += 1;
        }

        for i in count..capacity {
            let p = Peer::new_empty();
            slots.push(Arc::new(Mutex::new(p)));
        }

        log!(
            DEBUG,
            "* Peer store init count: {}, len: {}, capacity: {}\n",
            count,
            slots.len(),
            capacity
        );
        log!(
            DEBUG,
            "*****************************************************\n"
        );

        PeerStore {
            // slots: Arc::new(Mutex::new(slots)),
            slots,
            capacity,
        }
    }

    pub fn next(
        &self,
        last_idx: Option<usize>,
        filter: &(dyn Fn(&MutexGuard<Peer>) -> bool + Sync + Send),
    ) -> Option<(MutexGuard<'_, Peer>, usize)> {
        let slots = &self.slots;

        let start_idx = match last_idx {
            Some(i) => i + 1,
            None => 0,
        };

        let cap = self.capacity;

        for i in start_idx..start_idx + cap {
            let idx = i % cap;

            let peer = match slots.get(idx) {
                Some(p) => p,
                None => {
                    log!(
                        DEBUG,
                        "There is an empty slot. Something might be wrong\n"
                    );
                    return None;
                }
            };

            let peer = match peer.try_lock() {
                Ok(p) => p,
                Err(_) => continue,
            };

            if filter(&peer) {
                return Some((peer, idx))
            } else {
                continue;
            }
        }

        None
    }

    pub fn find(
        &self,
        filter: &(dyn Fn(&MutexGuard<Peer>) -> bool + Sync + Send),
    ) -> Option<(MutexGuard<'_, Peer>, usize)> {
        for (idx, p) in self.slots.iter().enumerate() {
            let peer = match p.try_lock() {
                Ok(p) => p,
                Err(_) => continue
            };

            if filter(&peer) {
                return Some((peer, idx));
            } else {
                continue;
            }
        }
        None
    }

    pub fn reserve(&self) -> Option<(MutexGuard<Peer>, usize)> {
        self.find(&|peer| {
            peer.status == Status::Empty
        })
    }
}
