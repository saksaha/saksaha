use std::collections::LinkedList;

use crate::common::SakResult;

pub struct Address {
    peer_id: String,
    endpoint: String,
}

impl Address {
    pub fn parse(url: String) -> SakResult<Address> {
        let a = url.split_at(5);
        let endpoint: Vec<&str> = url.split("@").collect();
        let endpoint = match endpoint.get(1) {
            Some(e) => e,
            None => {
                log!(
                    DEBUG,
                    "Cannot get endpoint out of url. \
                            Something might be wrong\n"
                );
                continue;
            }
        };
    }
}

pub struct AddressBook {
    pub addrs: LinkedList<Address>,
}

impl AddressBook {
    pub fn new(bootstrap_urls: Option<Vec<String>>) -> SakResult<AddressBook> {
        let default_urls = crate::default_bootstrap_urls!()
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        let bootstrap_urls = match bootstrap_urls {
            Some(u) => u,
            None => vec![],
        };

        let node_urls = [default_urls, bootstrap_urls].concat();

        let addrs = LinkedList::new();
        // addrs.push_back(1);

        for url in node_urls {}

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
