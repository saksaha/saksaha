pub mod peer_store;
mod bootstrap;

use std::cmp::PartialEq;
use crate::{common::{Error, Result}, err};
use logger::log;

#[derive(Debug, PartialEq)]
pub enum Status<E> {
    NotInitialized,

    DiscoverySuccess,

    HandshakeSuccess,

    HandshakeFail(E),
}

#[derive(Debug)]
pub struct Peer {
    pub ip: String,
    pub disc_port: u16,
    pub peer_op_port: u16,
    pub public_key_bytes: [u8; 65],
    pub rpc_port: u16,
    pub peer_id: String,
    pub status: Status<Error>,
}

impl Peer {
    pub fn new(
        peer_id: String,
        ip: String,
        disc_port: u16,
    ) -> Peer {
        Peer {
            ip,
            disc_port,
            peer_op_port: 0,
            public_key_bytes: [0; 65],
            rpc_port: 0,
            peer_id,
            status: Status::NotInitialized,
        }
    }

    pub fn parse(url: String) -> Result<Peer> {
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

        let addr = Peer::new(peer_id, ip, disc_port);

        Ok(addr)
    }
}
