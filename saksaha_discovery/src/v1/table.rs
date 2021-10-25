use super::address::Address;
use crate::DiscoveryError;
use log::{debug, info, warn};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::{mpsc, Mutex};
use rand::prelude::*;

pub struct Table {
    addrs: Vec<Address>,
    mutex: Mutex<bool>,
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

        let addrs = {
            let mut v = vec![];
            for url in urls {
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

                v.push(addr);
            }
            v
        };

        info!("*********************************************************");
        info!("* Discovery table bootstrapped");

        for (idx, addr) in addrs.iter().enumerate() {
            info!("* [{}] {}", idx, addr.short_url());
        }

        info!("* Address count: {}", addrs.len());
        info!("*********************************************************");

        let table = Table {
            addrs,
            mutex: Mutex::new(false),
        };

        Ok(table)
    }

    pub async fn next(self: Arc<Self>) {
        let guard = self.mutex.lock().await;

        let addrs = &self.addrs;
        let addr_count = addrs.len();
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..addr_count);

        addrs.get(idx);

        // self.addrs.get_mut(idx)
    }
}

// impl AddressIterator for Table {
//     fn next(&self) {

//     }
// }
