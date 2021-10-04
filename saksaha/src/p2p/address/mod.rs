pub mod status;
mod bootstrap;
pub mod address_book;

use crate::{common::Result, err};
use logger::log;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};
use status::Status;

#[derive(Debug)]
pub struct Address {
    pub peer_id: String,
    pub endpoint: String,
    pub fail_count: usize,
    pub status: Status<usize>,
}

impl Address {
    pub fn new(peer_id: String, endpoint: String) -> Address {
        let addr = Address {
            peer_id,
            endpoint,
            fail_count: 0,
            status: Status::NotInitialized,
        };
        addr
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

        let addr = Address {
            peer_id,
            endpoint,
            fail_count: 0,
            status: Status::NotInitialized,
        };

        Ok(addr)
    }
}
