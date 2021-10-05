pub mod address_book;
mod bootstrap;

use crate::{
    common::{Error, Result},
    err,
};
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
    pub ip: String,
    pub disc_port: u16,
    pub fail_count: usize,
    pub status: Status<Error>,
}

impl Address {
    pub fn new(peer_id: String, ip: String, disc_port: u16) -> Address {
        Address {
            peer_id,
            ip,
            disc_port,
            fail_count: 0,
            status: Status::NotDiscovered,
        }
    }

    pub fn new_empty() -> Address {
        Address {
            peer_id: "".into(),
            ip: "".into(),
            disc_port: 0,
            fail_count: 0,
            status: Status::Empty,
        }
    }

    pub fn parse(url: String) -> Result<Address> {
        let (peer_id, ip, disc_port) = {
            match url.get(6..) {
                Some(u) => match u.split_once('@') {
                    Some((peer_id, endpoint)) => {
                        match endpoint.split_once(":") {
                            Some((ip, port)) => (
                                peer_id.to_string(),
                                ip.to_string(),
                                port.to_string(),
                            ),
                            None => {
                                return err!("url may have illegal ip or port");
                            }
                        }
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

        let disc_port = match disc_port.parse::<u16>() {
            Ok(d) => d,
            Err(err) => {
                return err!(
                    "disc port cannot be converted to u16, err: {}",
                    err
                )
            }
        };

        let addr = Address::new(peer_id, ip, disc_port);

        Ok(addr)
    }
}
