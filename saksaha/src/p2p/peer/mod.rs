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
    pub ip: String,
    pub disc_port: u16,
    pub peer_op_port: u16,
    pub rpc_port: u16,
    pub peer_id: String,
    pub status: Status<Error>,
}

impl Peer {
    pub fn new(
        ip: String,
        peer_id: String,
        status: Status<Error>,
    ) -> Peer {
        Peer {
            ip,
            disc_port: 0,
            peer_op_port: 0,
            rpc_port: 0,
            peer_id,
            status,
        }
    }
}
