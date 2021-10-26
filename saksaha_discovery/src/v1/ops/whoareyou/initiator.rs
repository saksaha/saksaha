use crate::v1::{address::Address, table::Table};
use crypto::{Signature, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::io::{AsyncWriteExt};

#[derive(Error, Debug)]
pub enum WhoAreYouInitiateError {
    #[error("Connection failed, endpoint: {0}")]
    ConnectionFail(String),
}

pub struct WhoAreYouInitiator;

impl WhoAreYouInitiator {
    pub async fn run(
        table: Arc<Table>,
        addr: &Address,

    ) -> Result<(), WhoAreYouInitiateError> {
        let endpoint = addr.endpoint();

        let mut stream = match TcpStream::connect(endpoint.to_owned()).await {
            Ok(s) => {
                debug!("Successfully connected to endpoint, {}", endpoint);
                s
            }
            Err(err) => {
                return Err(WhoAreYouInitiateError::ConnectionFail(endpoint));
            }
        };

        // match WhoAreYouInitiator::initiate_who_are_you(&mut stream).await {
        //     Ok(_) => (),
        //     Err(err) => {
        //         peer.record_fail();

        //         return HandleStatus::WhoAreYouInitiateFail(err);
        //     }
        // };

        // let way_ack = match WhoAreYouInitiator::wait_for_ack(stream).await {
        //     Ok(w) => w,
        //     Err(err) => {
        //         peer.record_fail();

        //         // return HandleStatus::WhoAreYouAckReceiveFail(err);
        //     }
        // };

        // match self.handle_succeed_who_are_you(way_ack, peer).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::PeerUpdateFail(err),
        // };

        Ok(())
    }

    pub async fn initiate_who_are_you(
        stream: &mut TcpStream,
    ) -> Result<(), WhoAreYouInitiateError> {
        // let secret_key = &self.credential.secret_key;
        // let signing_key = SigningKey::from(secret_key);
        // let sig: Signature = signing_key.sign(msg::codec::MESSAGE);

        // let way = WhoAreYouMsg::new(
        //     msg::codec::Kind::Syn,
        //     sig,
        //     self.peer_op_port,
        //     self.credential.public_key_bytes,
        // );

        // let buf = match way.to_bytes() {
        //     Ok(b) => b,
        //     Err(err) => {
        //         let msg =
        //             format!("Error creating WhoAreYou request, err: {}", err);
        //         return Err(Error::new(msg));
        //     }
        // };

        // match stream.write_all(&buf).await {
        //     Ok(_) => Ok(()),
        //     Err(err) => {
        //         let msg = format!(
        //             "Error sending the whoAreYou buffer, err: {}, buf: {:?}",
        //             err,
        //             buf,
        //         );
        //         return Err(Error::new(msg));
        //     }
        // }

        Ok(())
    }
}
