use super::Address;
use crate::{common::Result, err};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

type MutexedAddress = Arc<Mutex<Address>>;

pub struct AddressBook {
    pub addrs: Arc<Mutex<Vec<MutexedAddress>>>,
    pub curr_idx: Mutex<usize>,
    pub capacity: usize,
}

impl AddressBook {
    pub fn new(bootstrap_urls: Option<Vec<String>>) -> AddressBook {
        let default_urls = crate::default_bootstrap_urls!()
            .into_iter()
            .map(|url| url.to_string())
            .collect::<Vec<String>>();

        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => vec![],
        };

        let node_urls = [default_urls, bootstrap_urls].concat();

        let mut addrs = Vec::new();
        let mut count = 0;

        log!(DEBUG, ">>>>>>>>>>>>>>>>>>>>>>\n");
        log!(DEBUG, "Address book\n");
        for url in node_urls {
            if let Ok(addr) = Address::parse(url) {
                log!(
                    DEBUG,
                    "Address book [{}]: {} @ {}\n",
                    count,
                    addr.peer_id,
                    addr.endpoint
                );
                let addr = Arc::new(Mutex::new(addr));
                addrs.push(addr);
                count += 1;
            }
        }

        log!(DEBUG, "Address book size: {}\n", count);
        log!(DEBUG, "<<<<<<<<<<<<<<<<<<<<<<\n");

        let book = AddressBook {
            addrs: Arc::new(Mutex::new(addrs)),
            curr_idx: Mutex::new(0),
            capacity: 100,
        };
        book
    }

    pub async fn next(
        &self,
        filter: Option<&(dyn Fn(MutexGuard<Address>) -> bool + Sync + Send)>,
    ) -> Option<(Arc<Mutex<Address>>, usize)> {
        let addrs = self.addrs.lock().await;
        let mut curr_idx = self.curr_idx.lock().await;
        let len = addrs.len();

        for i in *curr_idx..(*curr_idx + len) {
            let idx = i % len;

            match addrs.get(idx) {
                Some(addr) => {
                    let a = match addr.try_lock() {
                        Ok(a) => a,
                        Err(_) => continue,
                    };

                    if let Some(ref f) = filter {
                        if f(a) {
                            *curr_idx = idx;
                            return Some((addr.clone(), i));
                        } else {
                            continue;
                        }
                    } else {
                        *curr_idx = idx;
                        return Some((addr.clone(), i));
                    }
                }
                None => continue,
            };
        }

        *curr_idx = 0;
        None
    }

    pub async fn remove(&self, idx: usize) -> Result<Arc<Mutex<Address>>> {
        let mut addrs = self.addrs.lock().await;

        if idx <= addrs.len() {
            let addr = addrs.remove(idx);
            log!(DEBUG, "Address removed, idx: {}\n", idx);
            return Ok(addr);
        } else {
            return err!("Index out of bounds, idx:{}", idx);
        }
    }

    pub async fn len(&self) -> usize {
        let addrs = self.addrs.lock().await;
        addrs.len()
    }

    pub async fn reserve(&self) -> Option<Arc<Mutex<Address>>> {
        let mut addrs = self.addrs.lock().await;

        if addrs.len() < self.capacity {
            let addr = Address::new_empty();
            let addr = Arc::new(Mutex::new(addr));

            addrs.push(addr.clone());
            return Some(addr);
        }

        None
    }
}
