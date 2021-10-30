use std::sync::Arc;
use tokio::net::UdpSocket;
use crate::v1::{address::Address, DiscState};
use self::{initiator::WhoAreYouInitiator, receiver::WhoAreYouReceiver};
use thiserror::Error;

pub mod initiator;
pub mod msg;
pub mod receiver;

#[derive(Error, Debug)]
pub enum WhoAreYouError {
    #[error("Aborting, request to my endpoint: {0}")]
    MyEndpoint(String),

    #[error("Connection failed, endpoint: {0}, _err: {1}")]
    ConnectionFail(String, String),

    #[error("Cannot reserve tableNode, _err: {0}")]
    NodeReserveFail(String),

    #[error("Call already in progress, endpoint: {0}")]
    CallAlreadyInProgress(String),

    #[error("Couldn't sent msg through socket")]
    SendFail(#[from] std::io::Error),

    #[error("Cannot convert to byte, _err: {0}")]
    ByteConversionFail(String),

    #[error("Cannot create verifying key of remote, _err: {0}")]
    VerifiyingKeyFail(String),

    #[error("Signature is invalid, buf: {:?}, _err: {1}")]
    InvalidSignature(Vec<u8>, String),

    #[error("Failed to register node into map, endpoint: {0}, _err: {1}")]
    NodeRegisterFail(String, String),

    #[error("Couldn't parse WhoAreYou message, err: {0}")]
    MessageParseFail(String),

    #[error("Couldn't reserve node, table is full, endpoint: {0}, err: {1}")]
    TableIsFull(String, String),
}

pub struct WhoAreYouOperator {
    pub initiator: WhoAreYouInitiator,
    pub receiver: WhoAreYouReceiver,
}

impl WhoAreYouOperator {
    pub fn new(
        udp_socket: Arc<UdpSocket>,
        disc_state: Arc<DiscState>,
    ) -> WhoAreYouOperator {
        let initiator =
            WhoAreYouInitiator::new(udp_socket.clone(), disc_state.clone());

        let receiver =
            WhoAreYouReceiver::new(disc_state.clone(), udp_socket.clone());

        WhoAreYouOperator {
            initiator,
            receiver,
        }
    }
}

fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
    let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

    my_disc_endpoint == *endpoint
}
