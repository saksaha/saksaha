use super::{Peer, Status};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

pub struct Filter;

impl Filter {
    pub fn not_initialized(peer: &MutexGuard<Option<Peer>>) -> bool {
        if let Some(p) = &**peer {
            return p.status == Status::NotInitialized;
        }
        false
    }

    pub fn discovery_success(peer: &MutexGuard<Option<Peer>>) -> bool {
        if let Some(p) = &**peer {
            return p.status == Status::DiscoverySuccess;
        }
        false
    }
}

type MutexedPeer = Arc<Mutex<Option<Peer>>>;

pub struct PeerStore {
    mutex: Mutex<usize>,
    slots: Vec<MutexedPeer>,
    pub capacity: usize,
    pub curr_idx: Mutex<usize>,
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
                    Arc::new(Mutex::new(Some(p)))
                }
                Err(err) => {
                    log!(DEBUG, "Cannot parse url, url: {}, err: {}\n", u, err);
                    continue;
                }
            };

            slots.push(p);
            count += 1;
        }

        for _ in count..capacity {
            let empty_peer = Arc::new(Mutex::new(None));
            slots.push(empty_peer);
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
            mutex: Mutex::new(0),
            curr_idx: Mutex::new(0),
            slots,
            capacity,
        }
    }

    pub async fn next(
        &self,
        start_idx: Option<usize>,
        filter: &(dyn Fn(&MutexGuard<Option<Peer>>) -> bool + Sync + Send),
    ) -> Option<MutexGuard<'_, Option<Peer>>> {
        self.mutex.lock().await;

        let start_idx = match start_idx {
            Some(i) => i,
            None => 0,
        };

        let cap = self.capacity;
        let mut curr_idx = self.curr_idx.lock().await;
        let start_idx = *curr_idx + 1;

        for i in start_idx..start_idx + cap {
            let idx = i % cap;

            if let Some(p) = self.slots.get(idx) {
                let peer_lock = match p.try_lock() {
                    Ok(p) => p,
                    Err(_) => continue,
                };

                if filter(&peer_lock) {
                    *curr_idx = idx;

                    return Some(peer_lock);
                } else {
                    continue;
                }
            }
        }

        *curr_idx = 0;
        None
    }
}
