use super::msg::{WhoAreYouAck, WhoAreYouSyn};
use crate::v1::address::Address;
use crate::v1::ops::Message;
use crate::v1::table::{Node, NodeInner};
use crate::v1::DiscState;
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;

#[derive(Error, Debug)]
pub enum WhoareyouRecvError {
    #[error("Cannot convert to byte, _err: {err}")]
    ByteConversionFail { err: String },

    #[error("Can't send ack to my endpoint, endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Couldn't parse WhoAreYou message, err: {err}")]
    MessageParseFail { err: String },

    #[error(
        "Couldn't reserve node, table is full, endpoint:\
        {endpoint}, err: {err}"
    )]
    TableIsFull { endpoint: String, err: String },

    #[error("Couldn't sent msg through socket")]
    SendFail(#[from] std::io::Error),

    #[error("Can't add node to table, err: {err}")]
    TableAddFail { err: String },
}

pub(crate) struct WhoareyouReceive {
    disc_state: Arc<DiscState>,
    udp_socket: Arc<UdpSocket>,
}

impl WhoareyouReceive {
    pub fn new(
        udp_socket: Arc<UdpSocket>,
        disc_state: Arc<DiscState>,
    ) -> WhoareyouReceive {
        WhoareyouReceive {
            disc_state,
            udp_socket,
        }
    }

    pub async fn handle_who_are_you(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoareyouRecvError> {
        let endpoint = addr.endpoint();

        let table_node = match self.disc_state.table.try_reserve().await {
            Ok(n) => n,
            Err(err) => {
                return Err(WhoareyouRecvError::TableIsFull { endpoint, err })
            }
        };

        let way_syn = match WhoAreYouSyn::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoareyouRecvError::MessageParseFail { err });
            }
        };

        match self
            .disc_state
            .table
            .add(table_node, |mut inner| {
                *inner = NodeInner::Identified {
                    addr: addr.clone(),
                    sig: way_syn.way.sig,
                    p2p_port: way_syn.way.p2p_port,
                    public_key_bytes: way_syn.way.public_key_bytes,
                };
                inner
            })
            .await
        {
            Ok((public_key_bytes, endpoint)) => {
                debug!(
                    "Node is inserted, key: {:?}, endpoint: {:?}",
                    public_key_bytes, endpoint
                );
            }
            Err(err) => return Err(WhoareyouRecvError::TableAddFail { err }),
        };

        self.send_who_are_you_ack(addr).await?;

        Ok(())
    }

    pub async fn send_who_are_you_ack(
        &self,
        addr: Address,
    ) -> Result<(), WhoareyouRecvError> {
        let my_disc_port = self.disc_state.my_disc_port;
        let my_p2p_port = self.disc_state.my_p2p_port;
        let endpoint = addr.endpoint();

        if super::is_my_endpoint(my_disc_port, &endpoint) {
            return Err(WhoareyouRecvError::MyEndpoint { endpoint });
        }

        let sig = self.disc_state.id.sig();

        let way_ack = WhoAreYouAck::new(
            sig,
            my_p2p_port,
            self.disc_state.id.public_key_bytes(),
        );

        let buf = match way_ack.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return Err(WhoareyouRecvError::ByteConversionFail { err });
            }
        };

        self.udp_socket.send_to(&buf, endpoint.clone()).await?;

        debug!(
            "Successfully sent WhoAreYouAck to endpoint: {:?}, len: {}",
            &endpoint,
            buf.len(),
        );

        Ok(())
    }
}
