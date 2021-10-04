use crate::{common::Result, err};
use logger::log;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug)]
pub struct Address {
    pub peer_id: String,
    pub endpoint: String,
    pub fail_count: usize,
}

impl Address {
    pub fn parse(url: String) -> Result<Address> {
        let (peer_id, endpoint) = {
            match url.get(6..) {
                Some(u) => match u.split_once('@') {
                    Some((peer_id, endpoint)) => {
                        (peer_id.to_string(), endpoint.to_string())
                    }
                    None => {
                        return err!("url is not valid, url: {}", url);
                    }
                },
                None => {
                    return err!("url might be too short, url: {}", url);
                }
            }
        };

        let addr = Address {
            peer_id,
            endpoint,
            fail_count: 0,
        };

        Ok(addr)
    }
}

pub struct AddressBook {
    pub addrs: Arc<Mutex<Vec<Arc<Mutex<Address>>>>>,
    pub curr_idx: Mutex<usize>,
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
                addrs.push(Arc::new(Mutex::new(addr)));
                count += 1;
            }
        }

        log!(DEBUG, "Address book size: {}\n", count);
        log!(DEBUG, "<<<<<<<<<<<<<<<<<<<<<<\n");

        let book = AddressBook {
            addrs: Arc::new(Mutex::new(addrs)),
            curr_idx: Mutex::new(0),
        };
        book
    }

    pub async fn next(
        &self,
        filter: Option<Box<Fn()>>,
    ) -> Option<(Arc<Mutex<Address>>, usize)> {
        let addrs = self.addrs.lock().await;
        let mut curr_idx = self.curr_idx.lock().await;
        let len = addrs.len();

        for i in *curr_idx..(*curr_idx + len) {
            let idx = i % len;

            let addr = match addrs.get(idx) {
                Some(addr) => {
                    if let Some(ref f) = filter {
                        f();
                        Some(addr)
                    } else {
                        Some(addr)
                    }
                },
                None => {
                    None
                }
            };
        }

        None

        // if let Some(a) = addrs.get(*idx) {
        //     let p = Some((a.clone(), idx.to_owned()));
        //     *idx += 1;
        //     return p;
        // } else {
        //     *idx = 0;
        //     match addrs.get(*idx) {
        //         Some(a) => {
        //             let p = Some((a.clone(), idx.to_owned()));
        //             *idx += 1;
        //             return p;
        //         }
        //         None => {
        //             return None;
        //         }
        //     }
        // };
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
}

#[macro_export]
macro_rules! default_bootstrap_urls {
    () => {
        vec![
            "sak://041220dd105993ecd5a854341e5d3694f3fb6fd35d91b55fe3f4f84c83d06fa2890789c485fd532f4a5f03e4e94148398a5dbaf2c83b900c6d57f57bb8286e0fa9@127.0.0.1:35518",
            // "sak://041efae14ece202c@127.0.0.1:35519"
        ]
    };
}
