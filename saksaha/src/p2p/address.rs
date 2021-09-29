use std::collections::LinkedList;

use crate::{common::SakResult, err_res};

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

        let addr = Address {
            peer_id,
            endpoint,
        };

        Ok(addr)
    }
}

pub struct AddressBook {
    pub addrs: Vec<Address>,
}

impl AddressBook {
    pub fn new(bootstrap_urls: Option<Vec<String>>) -> SakResult<AddressBook> {
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

        for url in node_urls {
            if let Ok(addr) = Address::parse(url) {
                addrs.push(addr);
            }
        }

        let b=  addrs.iter().next().unwrap();

        // let address_book: Vec<String> = match bootstrap_urls {
        //     Some(b) => b,
        //     None => vec!(),
        // };

        // for (idx, addr) in address_book.iter().enumerate() {
        //     log!(DEBUG, "address book [{}]: {}\n", idx, addr);
        // }

        let book = AddressBook { addrs };

        Ok(book)
    }
}

#[macro_export]
macro_rules! default_bootstrap_urls {
    () => {
        vec!["sak://041efae14ece202c@127.0.0.1:35518"]
    };
}
