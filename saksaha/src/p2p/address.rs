use crate::{common::SakResult, err_res};
use logger::log;
use std::collections::LinkedList;

pub struct Address {
    peer_id: String,
    endpoint: String,
}

impl Address {
    pub fn parse(url: String) -> SakResult<Address> {
        let (peer_id, endpoint) = {
            match url.get(6..) {
                Some(u) => match u.split_once('@') {
                    Some((peer_id, endpoint)) => {
                        (peer_id.to_string(), endpoint.to_string())
                    }
                    None => {
                        return err_res!("url is not valid, url: {}", url);
                    }
                },
                None => {
                    return err_res!("url might be too short, url: {}", url);
                }
            }
        };

        let addr = Address { peer_id, endpoint };

        Ok(addr)
    }
}

pub struct AddressBook {
    pub addrs: Vec<Address>,
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
                addrs.push(addr);
                count += 1;
            }
        }

        log!(DEBUG, "Address book size: {}\n", count);
        log!(DEBUG, "<<<<<<<<<<<<<<<<<<<<<<\n");

        let book = AddressBook { addrs };
        book
    }
}

#[macro_export]
macro_rules! default_bootstrap_urls {
    () => {
        vec!["sak://041efae14ece202c@127.0.0.1:35518"]
    };
}
