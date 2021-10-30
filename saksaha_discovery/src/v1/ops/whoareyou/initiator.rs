use super::msg::{WhoAreYou, WhoAreYouAck, WhoAreYouSyn, SAKSAHA};
use crate::v1::active_calls::Traffic;
use crate::v1::ops::whoareyou::WhoAreYouError;
use crate::v1::ops::{Message, Opcode};
use crate::v1::table::{Record, TableNode};
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use crypto::{Crypto, Signature, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::{AsyncWriteExt, Interest};
use tokio::net::{TcpStream, UdpSocket};

// #[derive(Error, Debug)]
// pub enum WhoAreYouInitError {
//     #[error("Aborting, request to my endpoint: {0}")]
//     MyEndpoint(String),

//     #[error("Connection failed, endpoint: {0}, _err: {1}")]
//     ConnectionFail(String, String),

//     #[error("Cannot reserve tableNode, _err: {0}")]
//     NodeReserveFail(String),

//     #[error("Call already in progress, endpoint: {0}")]
//     CallAlreadyInProgress(String),

//     #[error("Couldn't sent msg through socket")]
//     SendFail(#[from] std::io::Error),

//     #[error("Cannot convert to byte, _err: {0}")]
//     ByteConversionFail(String),

//     #[error("Cannot parse WAY ack, _err: {0}")]
//     AckParseFail(String),

//     #[error("Cannot create verifying key of remote, _err: {0}")]
//     VerifiyingKeyFail(String),

//     #[error("Signature is invalid, buf: {:?}, _err: {1}")]
//     InvalidSignature(Vec<u8>, String),

//     #[error("Failed to register node into map, endpoint: {0}, _err: {1}")]
//     NodeRegisterFail(String, String),
// }

pub struct WhoAreYouInitiator {
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
    ) -> Result<(), WhoAreYouError> {
        let my_disc_port = self.disc_state.my_disc_port;
        let my_p2p_port = self.disc_state.my_p2p_port;

        let endpoint = addr.endpoint();

        if super::is_my_endpoint(my_disc_port, &endpoint) {
            return Err(WhoAreYouError::MyEndpoint(endpoint));
        }

        let table_node = {
            let node = match self.disc_state.table.find(&endpoint).await {
                Some(n) => n,
                None => match self.disc_state.table.reserve().await {
                    Ok(n) => n,
                    Err(err) => {
                        return Err(WhoAreYouError::NodeReserveFail(err));
                    }
                },
            };
            node
        };

        let secret_key = self.disc_state.id.secret_key();
        let signing_key = SigningKey::from(secret_key);
        let sig = Crypto::make_sign(signing_key, SAKSAHA);

        let way_syn = WhoAreYouSyn::new(
            sig,
            my_p2p_port,
            self.disc_state.id.public_key_bytes(),
        );

        let buf = match way_syn.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return Err(WhoAreYouError::ByteConversionFail(err));
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
    ) -> Result<(), WhoAreYouError> {
        let endpoint = addr.endpoint();

        let table_node = {
            let node = match self.disc_state.table.find(&endpoint).await {
                Some(n) => n,
                None => match self.disc_state.table.try_reserve().await {
                    Ok(n) => n,
                    Err(err) => {
                        return Err(WhoAreYouError::TableIsFull(
                            endpoint, err,
                        ));
                    }
                },
            };
            node
        };

        let way_syn = match WhoAreYouSyn::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoAreYouError::MessageParseFail(err));
            }
        };

        let mut table_node = table_node.lock().await;
        table_node.record = Some(Record {
            sig: way_syn.way.sig,
            p2p_port: way_syn.way.p2p_port,
            public_key_bytes: way_syn.way.public_key_bytes,
        });

        // self.send_who_are_you_ack(addr).await?;

        Ok(())
    }

    // pub async fn initiate_who_are_you(
    //     &self,
    //     endpoint: String,
    //     my_p2p_port: u16,
    // ) -> Result<(), WhoAreYouInitError> {
    //     let secret_key = self.disc_state.id.secret_key();
    //     let signing_key = SigningKey::from(secret_key);
    //     let sig = Crypto::make_sign(signing_key, SAKSAHA);

    //     let way = WhoAreYouMsg::new(
    //         Opcode::WhoAreYou,
    //         sig,
    //         my_p2p_port,
    //         self.disc_state.id.public_key_bytes(),
    //     );

    //     let buf = match way.to_bytes() {
    //         Ok(b) => b,
    //         Err(err) => {
    //             return Err(WhoAreYouInitError::ByteConversionFail(err));
    //         }
    //     };

    //     match self.udp_socket.send_to(&buf, endpoint.clone()).await {
    //         Ok(_) => (),
    //         Err(err) => {
    //             return Err(WhoAreYouInitError::WaySendFail(
    //                 endpoint,
    //                 err.to_string(),
    //             ));
    //         }
    //     };

    //     Ok(())
    // }

}
