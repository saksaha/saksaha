use super::address::Address;
use crate::DiscoveryError;
use log::{debug, warn, info};
use std::collections::HashMap;
use tokio::sync::{mpsc, Mutex};

pub struct Table {
    addrs: Mutex<Vec<Address>>,
}

impl Table {
    pub fn init(
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Table, String> {
        let (tx, rx) = mpsc::channel::<usize>(10);

        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => Vec::new(),
        };

        let default_bootstrap_urls: Vec<String> = default_bootstrap_urls
            .lines()
            .map(|l| l.to_string())
            .collect();

        let urls = [bootstrap_urls, default_bootstrap_urls].concat();
        let url_count = urls.len();

        if url_count > 0 {
            debug!(
                "Initializing discovery bootstrap urls, candidates: {}",
                url_count
            );
        }

        let addrs = {
            let mut v = vec![];
            for (idx, url) in urls.iter().enumerate() {
                let addr = match Address::parse(url.clone()) {
                    Ok(a) => a,
                    Err(err) => {
                        warn!(
                            "Discarding url failed to parse, url: {}, \
                                err: {:?}",
                            url.clone(),
                            err,
                        );

                        continue;
                    }
                };

                info!("Discovery address [{}], {:?}", idx, addr);

                v.push(addr);
            }
            v
        };

        let table = Table {
            addrs: Mutex::new(addrs),
        };

        Ok(table)
    }
}

// impl AddressIterator for Table {
//     fn next(&self) {

//     }
// }
