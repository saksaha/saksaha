use std::cmp::PartialEq;
use crate::common::Error;

pub mod peer_store;

#[derive(Debug, PartialEq)]
pub enum Status<E> {
    NotInitialized,

    DiscoverySuccess,

    HandshakeSuccess,

    HandshakeFail(E),
}

#[derive(Debug)]
pub struct Peer {
    pub endpoint: String,
    pub peer_id: String,
    pub status: Status<Error>,
}

impl Peer {
    pub fn new(
        endpoint: String,
        peer_id: String,
        status: Status<Error>,
    ) -> Peer {
        Peer {
            endpoint,
            peer_id,
            status,
        }
    }
}
