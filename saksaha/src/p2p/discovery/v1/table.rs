use logger::log;
use tokio::sync::{Mutex, mpsc};
use crate::common::Result;
use super::address::Address;

pub struct Table {
    addrs: Mutex<Vec<Address>>,
}

impl Table {
    pub fn new(
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Table> {
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
            log!(
                DEBUG,
                "Initializing discovery bootstrap urls, candidates: {}\n",
                url_count
            );
        }

        let addrs = {
            let mut v = vec!();
            for (idx, url) in urls.iter().enumerate() {
                let addr = match Address::parse(url.clone()) {
                    Ok(a) => a,
                    Err(err) => {
                        log!(
                            DEBUG,
                            "Discarding url failed to parse, url: {}, err: {}\n",
                            url.clone(),
                            err
                        );

                        continue;
                    }
                };

                log!(DEBUG, "Discovery address [{}], {:?}\n", idx, addr);

                v.push(addr);
            }
            v
        };

        let table = Table {
            addrs: Mutex::new(addrs),
        };

        Ok(table)
    }

    pub async fn next() {

    }

}
