use super::msg::{WhoAreYouAck, WhoAreYouSyn, SAKSAHA};
use crate::v1::address::Address;
use crate::v1::ops::whoareyou::WhoAreYouError;
use crate::v1::ops::Message;
use crate::v1::table::{Record, TableNode};
use crate::v1::DiscState;
use crypto::{Crypto, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::UdpSocket;

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
                        return Err(WhoAreYouError::TableIsFull(endpoint, err));
                    }
                },
            };
            node
        };

        let way_ack = match WhoAreYouAck::parse(buf) {
            Ok(m) => m,
            Err(err) => {
                return Err(WhoAreYouError::MessageParseFail(err));
            }
        };

        let mut table_node = table_node.lock().await;
        table_node.record = Some(Record {
            sig: way_ack.way.sig,
            p2p_port: way_ack.way.p2p_port,
            public_key_bytes: way_ack.way.public_key_bytes,
        });

        Ok(())
    }
}
