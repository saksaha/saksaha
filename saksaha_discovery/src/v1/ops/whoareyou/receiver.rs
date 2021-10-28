use crate::v1::active_calls::Traffic;
use crate::v1::ops::Opcode;
use crate::v1::table::TableNode;
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use crypto::{Crypto, Signature, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncWriteExt, Interest};
use tokio::net::{TcpStream, UdpSocket};
use super::msg::{WhoAreYouAckMsg, WhoAreYouMsg, SAKSAHA};

#[derive(Error, Debug)]
pub enum WhoAreYouInitError {
    #[error("Aborting, request to my endpoint: {0}")]
    MyEndpoint(String),

    #[error("Connection failed, endpoint: {0}, _err: {1}")]
    ConnectionFail(String, String),

    #[error("Cannot reserve tableNode, _err: {0}")]
    NodeReserveFail(String),

    #[error("Call already in progress, endpoint: {0}")]
    CallAlreadyInProgress(String),

    #[error("Couldn't send WhoAreYou msg, endpoint: {0}, _err: {1}")]
    WaySendFail(String, String),

    #[error("Cannot convert to byte, _err: {0}")]
    ByteConversionFail(String),

    #[error("Cannot parse WAY ack, _err: {0}")]
    AckParseFail(String),

    #[error("Cannot create verifying key of remote, _err: {0}")]
    VerifiyingKeyFail(String),

    #[error("Signature is invalid, buf: {:?}, _err: {1}")]
    InvalidSignature(Vec<u8>, String),

    #[error("Failed to register node into map, endpoint: {0}, _err: {1}")]
    NodeRegisterFail(String, String),
}

pub struct WhoAreYouReceiver {
    disc_state: Arc<DiscState>,
}

impl WhoAreYouReceiver {
    pub fn new(disc_state: Arc<DiscState>) -> WhoAreYouReceiver {
        WhoAreYouReceiver {
            disc_state,
        }
    }
}
