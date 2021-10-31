use super::msg::{
    WhoAreYou, WhoAreYouAck, WhoAreYouSyn, P2P_PORT_LEN, SAKSAHA,
};
use crate::v1::ops::{Message, Opcode};
use crate::v1::table::{Record, TableNode};
use crate::v1::task_queue::{Task, TaskQueue};
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;

#[derive(Error, Debug)]
pub enum WhoAreYouRecvError {
    #[error("Cannot convert to byte, _err: {err}")]
    ByteConversionFail { err: String },

    #[error("Can't send ack to my endpoint, endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },

    #[error("Couldn't parse WhoAreYou message, err: {err}")]
    MessageParseFail { err: String },

    #[error(
        "Couldn't reserve node, table is full, endpoint: \
        {endpoint}, err: {err}"
    )]
    TableIsFull { endpoint: String, err: String },

    #[error("Couldn't sent msg through socket")]
    SendFail(#[from] std::io::Error),
}

pub struct WhoAreYouReceiver {
    disc_state: Arc<DiscState>,
    udp_socket: Arc<UdpSocket>,
}

impl WhoAreYouReceiver {
    pub fn new(
        disc_state: Arc<DiscState>,
        udp_socket: Arc<UdpSocket>,
    ) -> WhoAreYouReceiver {
        WhoAreYouReceiver {
            disc_state,
            udp_socket,
        }
    }

    pub async fn handle_who_are_you(
        &self,
        addr: Address,
        buf: &[u8],
    ) -> Result<(), WhoAreYouRecvError> {
        let endpoint = addr.endpoint();

        let table_node =
            match self.disc_state.table.find_or_try_reserve(&endpoint).await {
                Ok(n) => n,
                Err(err) => {
                    return Err(WhoAreYouRecvError::TableIsFull {
                        endpoint,
                        err,
                    })
                }
            };

        let way_syn = match WhoAreYouSyn::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoAreYouRecvError::MessageParseFail { err });
            }
        };

        // table_node

        // let mut table_node = table_node.lock().await;
        // table_node.record = Some(Record {
        //     sig: way_syn.way.sig,
        //     p2p_port: way_syn.way.p2p_port,
        //     public_key_bytes: way_syn.way.public_key_bytes,
        // });

        self.send_who_are_you_ack(addr).await?;

        Ok(())
    }

    pub async fn send_who_are_you_ack(
        &self,
        addr: Address,
    ) -> Result<(), WhoAreYouRecvError> {
        let my_disc_port = self.disc_state.my_disc_port;
        let my_p2p_port = self.disc_state.my_p2p_port;
        let endpoint = addr.endpoint();

        if super::is_my_endpoint(my_disc_port, &endpoint) {
            return Err(WhoAreYouRecvError::MyEndpoint { endpoint });
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
                return Err(WhoAreYouRecvError::ByteConversionFail { err });
            }
        };

        self.udp_socket.send_to(&buf, endpoint.clone()).await?;

        Ok(())
    }
}
