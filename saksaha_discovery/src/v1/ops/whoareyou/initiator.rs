use crate::v1::active_calls::Traffic;
use crate::v1::DiscState;
use crate::v1::{address::Address, table::Table};
use crypto::{Crypto, Signature, SigningKey};
use log::debug;
use std::sync::Arc;
use thiserror::Error;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

use super::msg::{MsgKind, WhoAreYouAckMsg, WhoAreYouMsg, SAKSAHA};

#[derive(Error, Debug)]
pub enum WhoAreYouInitError {
    #[error("Aborting, my endpoint: {0}")]
    MyEndpoint(String),

    #[error("Connection failed, endpoint: {0}, _err: {1}")]
    ConnectionFail(String, String),

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
}

pub struct WhoAreYouInitiator;

impl WhoAreYouInitiator {
    pub async fn run(
        state: Arc<DiscState>,
        addr: &Address,
        my_disc_port: u16,
        my_p2p_port: u16,
    ) -> Result<(), WhoAreYouInitError> {
        let endpoint = addr.endpoint();

        let active_calls = state.active_calls.clone();

        if active_calls.contain(&endpoint).await {
            return Err(WhoAreYouInitError::CallAlreadyInProgress(endpoint));
        } else {
            active_calls
                .insert(endpoint.to_string(), Traffic::OutBound)
                .await;
        }

        let result = WhoAreYouInitiator::_run(
            state,
            endpoint.to_string(),
            my_disc_port,
            my_p2p_port,
        )
        .await;

        active_calls.remove(&endpoint).await;
        result
    }

    async fn _run(
        state: Arc<DiscState>,
        endpoint: String,
        my_disc_port: u16,
        my_p2p_port: u16,
    ) -> Result<(), WhoAreYouInitError> {
        if WhoAreYouInitiator::is_my_endpoint(my_disc_port, &endpoint) {
            return Err(WhoAreYouInitError::MyEndpoint(endpoint));
        }

        let mut stream = match TcpStream::connect(endpoint.clone()).await {
            Ok(s) => {
                debug!("Successfully connected to endpoint, {}", endpoint);
                s
            }
            Err(err) => {
                return Err(WhoAreYouInitError::ConnectionFail(
                    endpoint,
                    err.to_string(),
                ));
            }
        };

        WhoAreYouInitiator::initiate_who_are_you(
            state.clone(),
            &mut stream,
            endpoint.clone(),
            my_p2p_port,
        )
        .await?;

        let way_ack =
            WhoAreYouInitiator::wait_for_ack(stream, &endpoint).await?;

        // match self.handle_succeed_who_are_you(way_ack, peer).await {
        //     Ok(_) => (),
        //     Err(err) => return HandleStatus::PeerUpdateFail(err),
        // };

        Ok(())
    }

    fn is_my_endpoint(my_disc_port: u16, endpoint: &String) -> bool {
        let my_disc_endpoint = format!("127.0.0.1:{}", my_disc_port);

        my_disc_endpoint == *endpoint
    }

    pub async fn initiate_who_are_you(
        state: Arc<DiscState>,
        stream: &mut TcpStream,
        endpoint: String,
        my_p2p_port: u16,
    ) -> Result<(), WhoAreYouInitError> {
        let secret_key = state.id.secret_key();
        let signing_key = SigningKey::from(secret_key);
        let sig = Crypto::make_sign(signing_key, SAKSAHA);

        let way = WhoAreYouMsg::new(
            MsgKind::Syn,
            sig,
            my_p2p_port,
            state.id.public_key_bytes(),
        );

        let buf = match way.to_bytes() {
            Ok(b) => b,
            Err(err) => {
                return Err(WhoAreYouInitError::ByteConversionFail(err));
            }
        };

        match stream.write_all(&buf).await {
            Ok(_) => (),
            Err(err) => {
                return Err(WhoAreYouInitError::WaySendFail(
                    endpoint,
                    err.to_string(),
                ));
            }
        };

        Ok(())
    }

    pub async fn wait_for_ack(
        mut stream: TcpStream,
        endpoint: &String,
    ) -> Result<WhoAreYouAckMsg, WhoAreYouInitError> {
        let way_ack = match WhoAreYouAckMsg::parse(&mut stream).await {
            Ok(w) => w,
            Err(err) => {
                return Err(WhoAreYouInitError::AckParseFail(err));
            }
        };

        let verifying_key = match Crypto::convert_public_key_to_verifying_key(
            way_ack.way.public_key_bytes,
        ) {
            Ok(v) => v,
            Err(err) => {
                return Err(WhoAreYouInitError::VerifiyingKeyFail(err));
            }
        };
        let sig = way_ack.way.sig;

        match Crypto::verify(verifying_key, SAKSAHA, &sig) {
            Ok(_) => (),
            Err(err) => {
                return Err(WhoAreYouInitError::InvalidSignature(
                    way_ack.way.raw,
                    err,
                ))
            }
        }

        Ok(way_ack)
    }
}
