mod bootstrap;
pub mod address_book;

use crate::{common::{Error, Result}, err};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

#[derive(Debug, PartialEq)]
pub enum Status<E> {
    // Empty value, used for reserving
    Empty,

    // Not discovered addresses need to pass "discovery", a.k.a "Who are you"
    NotDiscovered,

    DiscoverySuccess,

    DiscoveryFail(E),

    // HandshakeSucceed,

    // HandshakeFail(C),
}

#[derive(Debug)]
pub struct Address {
    pub peer_id: String,
    pub endpoint: String,
    pub fail_count: usize,
    pub status: Status<Error>,
}

impl Address {
    pub fn new(peer_id: String, endpoint: String) -> Address {
        Address {
            peer_id,
            endpoint,
            fail_count: 0,
            status: Status::NotDiscovered,
        }
    }

    pub fn new_empty() -> Address {
        Address {
            peer_id: "".into(),
            endpoint: "".into(),
            fail_count: 0,
            status: Status::Empty,
        }
    }

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

        let addr = Address::new(peer_id, endpoint);

        Ok(addr)
    }
}
