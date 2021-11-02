use super::msg::{WhoAreYouAck, WhoAreYouSyn};
use crate::v1::address::Address;
use crate::v1::ops::Message;
use crate::v1::table::{NodeInner};
use crate::v1::DiscState;
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;

#[derive(Error, Debug)]
pub enum WhoAreYouInitError {
    #[error("Aborting, request to my endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Cannot reserve Node, _err: {err}")]
    NodeReserveFail { err: String },

    #[error("Couldn't sent msg through socket")]
    SendFail(#[from] std::io::Error),

    #[error("Cannot convert to byte, _err: {err}")]
    ByteConversionFail { err: String },

    #[error("Cannot create verifying key of remote, _err: {err}")]
    VerifiyingKeyFail { err: String },

    #[error("Signature is invalid, buf: {buf:?}, _err: {err}")]
    InvalidSignature { buf: Vec<u8>, err: String },

    #[error(
        "Failed to register node into map, endpoint: {endpoint}, _err: {err}"
    )]
    NodeRegisterFail { endpoint: String, err: String },

    #[error("Can't parse WhoAreYou message, err: {err}")]
    MessageParseFail { err: String },

    #[error(
        "Can't reserve node, table is full, endpoint: {endpoint}, \
        err: {err}"
    )]
    TableIsFull { endpoint: String, err: String },

    #[error("Can't add node to table, err: {err}")]
    TableAddFail { err: String },
}

pub(crate) struct WhoAreYouInitiator {
    udp_socket: Arc<UdpSocket>,
    disc_state: Arc<DiscState>,
}

impl WhoAreYouInitiator {
    pub fn new(
        udp_socket: Arc<UdpSocket>,
        disc_state: Arc<DiscState>,
    ) -> WhoAreYouInitiator {
        WhoAreYouInitiator {
            udp_socket,
            disc_state,
        }
    }

    pub async fn send_who_are_you(
        &self,
        addr: Address,
    ) -> Result<(), WhoAreYouInitError> {
        let my_disc_port = self.disc_state.my_disc_port;
        let my_p2p_port = self.disc_state.my_p2p_port;

        let endpoint = addr.endpoint();

        if super::is_my_endpoint(my_disc_port, &endpoint) {
            return Err(WhoAreYouInitError::MyEndpoint { endpoint });
        }

        let my_sig = self.disc_state.id.sig();
        let my_public_key_bytes = self.disc_state.id.public_key_bytes();

        let way_syn =
            WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

        let buf = match way_syn.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return Err(WhoAreYouInitError::ByteConversionFail { err });
            }
        };

        self.udp_socket.send_to(&buf, endpoint.clone()).await?;

        debug!(
            "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
            &endpoint,
            buf.len()
        );

        Ok(())
    }

    pub async fn handle_who_are_you_ack(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoAreYouInitError> {
        let endpoint = addr.endpoint();

        let table_node = match self.disc_state.table.try_reserve().await {
            Ok(n) => n,
            Err(err) => {
                return Err(WhoAreYouInitError::TableIsFull { endpoint, err })
            }
        };

        let way_ack = match WhoAreYouAck::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoAreYouInitError::MessageParseFail { err });
            }
        };

        match self
            .disc_state
            .table
            .add(table_node, |mut n| {
                *n = NodeInner::Identified {
                    addr: addr.clone(),
                    sig: way_ack.way.sig,
                    p2p_port: way_ack.way.p2p_port,
                    public_key_bytes: way_ack.way.public_key_bytes,
                };
                n
            })
            .await
        {
            Ok((public_key_bytes, endpoint)) => {
                debug!(
                    "Node is inserted, key: {:?}, endpoint: {:?}",
                    public_key_bytes, endpoint
                );
            }
            Err(err) => return Err(WhoAreYouInitError::TableAddFail { err }),
        };

        Ok(())
    }
}
