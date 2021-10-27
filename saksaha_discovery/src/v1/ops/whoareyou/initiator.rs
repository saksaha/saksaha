use crate::v1::active_calls::Traffic;
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use crypto::{Signature, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

#[derive(Error, Debug)]
pub enum WhoAreYouInitError {
    #[error("Connection failed, endpoint: {0}")]
    ConnectionFail(String),

    #[error("Call already in pgoress, endpoint: {0}")]
    CallAlreadyInProgress(String),
}

pub struct WhoAreYouInitiator;

impl WhoAreYouInitiator {
    pub async fn run(
        state: Arc<DiscState>,
        addr: &Address,
    ) -> Result<(), WhoAreYouInitError> {
        let endpoint = addr.endpoint();

        let active_calls = state.active_calls.clone();

        if active_calls.contain(&endpoint).await {
            active_calls.insert(endpoint.to_string(), Traffic::OutBound).await;
        } else {
            return Err(WhoAreYouInitError::CallAlreadyInProgress(
                endpoint,
            ));
        }

        let result = WhoAreYouInitiator::_run(state, addr).await;

        active_calls.remove(&endpoint).await;
        result
    }

    async fn _run(
        state: Arc<DiscState>,
        addr: &Address,
    ) -> Result<(), WhoAreYouInitError> {
        // let mut stream = match TcpStream::connect(endpoint.to_owned()).await {
        //     Ok(s) => {
        //         debug!("Successfully connected to endpoint, {}", endpoint);
        //         s
        //     }
        //     Err(err) => {
        //         return Err(WhoAreYouInitiateError::ConnectionFail(endpoint));
        //     }
        // };

        // match WhoAreYouInitiator::initiate_who_are_you(&mut stream, state).await
        // {
        //     Ok(_) => (),
        //     Err(err) => {
        //         // peer.record_fail();

        //         // return HandleStatus::WhoAreYouInitiateFail(err);
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
        state: Arc<DiscState>,
    ) -> Result<(), WhoAreYouInitError> {
        println!("33, state {:?}", state.id.public_key_bytes());
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
