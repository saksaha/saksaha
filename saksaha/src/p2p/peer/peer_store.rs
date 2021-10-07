use super::{Peer, Status};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub struct Filter;

impl Filter {
    pub fn not_initialized(peer: &mut Peer) -> bool {
        return peer.status == Status::NotInitialized;
    }

    pub fn discovery_success(peer: &mut Peer) -> bool {
        return peer.status == Status::DiscoverySuccess;
    }
}

pub struct PeerStore {
    slots: Vec<Peer>,
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
                    p
                    // Arc::new(Mutex::new(Some(p)))
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
            slots.push(p);
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

    pub async fn next(
        &mut self,
        start_idx: Option<usize>,
        filter: &(dyn Fn(&mut Peer) -> bool + Sync + Send),
    ) -> Option<(&mut Peer, usize)> {
        // let mut slots = self.slots.clone().lock().await;
        let slots = &mut self.slots;

        let start_idx = match start_idx {
            Some(i) => i,
            None => 0,
        };

        let cap = self.capacity;

        for i in start_idx..start_idx + cap {
            let idx = i % cap;

            let mut peer = match slots.get_mut(idx) {
                Some(p) => p,
                None => {
                    log!(
                        DEBUG,
                        "There is an empty slot. Something might be wrong\n"
                    );
                    return None;
                }
            };

            if filter(peer) {
                let p = slots.get_mut(idx).unwrap();
                return Some((p, idx));
            } else {
                continue;
            }
        }

        None
    }

    pub async fn find(
        &self,
        filter: &(dyn Fn(&MutexGuard<Option<Peer>>) -> bool + Sync + Send),
    ) -> Option<(MutexGuard<'_, Option<Peer>>, usize)> {
        // self.mutex.lock().await;

        // for p in self.slots.clone() {
        //     let a = p.clone();
        // };
        // return false;
        None
    }
}
