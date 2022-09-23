use crate::HandshakeMsg;
use crate::{Conn, Msg, Transport};
use futures::SinkExt;
use futures::StreamExt;
use sak_p2p_id::Identity;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("Could not intilize handshake msg, err: {err}")]
    HandshakeMsgInitFail { err: String },

    #[error("P2P Port may not be provided")]
    InvalidP2PEndpoint,

    #[error("Cannot send request to myself, p2p_endpoint: {p2p_endpoint}")]
    MyEndpoint { p2p_endpoint: String },

    #[error("Cannot create tcp connection to endpoint, err: {err}")]
    ConnectionFail { err: String },

    #[error("Cannot retrieve peer address, err: {err}")]
    PeerAddressNotRetrievable { err: String },

    #[error("Cannot write frame (data) into connection, err: {err}")]
    FrameWriteFail { err: String },

    #[error("Data received may not be the entire frame intended, err: {err}")]
    InvalidFrame { err: String },

    #[error("Waited for Handshake ack but something else has arrived")]
    HandshakeAckWrongArrived,

    #[error("Failed to parse Handshake ack msg, err: {err}")]
    HandshakeAckParseFail { err: String },

    #[error("Handshake ack is not arrived")]
    HandshakeAckNotYetArrived,

    #[error(
        "Could not create a public key, err: {err}, \
         public_key: {public_key}"
    )]
    PublicKeyCreateFail { err: String, public_key: String },

    #[error("Peer is already mapped")]
    PeerAlreadyMapped,

    #[error(
        "Peer node is being used by another process (task), \
        public_key: {public_key}, err: {err}"
    )]
    PeerNodeAlreadyInUse { public_key: String, err: String },

    #[error("Peer node is empty.")]
    EmptyPeerNode,

    #[error("No available empty node in the addr table")]
    EmptyNodeNotAvailable,

    #[error("PeerNode has an unknown addr")]
    NotKnownAddr,

    #[error("Could not connect connection, err: {err}")]
    ConnectionCreateFail { err: String },
}

pub struct HandshakeInitArgs {
    pub identity: Arc<Identity>,
    pub conn: Conn,
    pub public_key_str: String,
}

pub async fn initiate_handshake(
    handshake_init_args: HandshakeInitArgs,
) -> Result<Transport, HandshakeInitError> {
    let HandshakeInitArgs {
        identity,
        mut conn,
        public_key_str,
    } = handshake_init_args;

    let handshake_msg = match HandshakeMsg::new(
        identity.p2p_port,
        identity.credential.public_key_str.clone(),
        public_key_str,
    ) {
        Ok(h) => h,
        Err(err) => {
            return Err(HandshakeInitError::HandshakeMsgInitFail { err });
        }
    };

    match conn.socket.send(Msg::HandshakeSyn(handshake_msg)).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeInitError::FrameWriteFail {
                err: err.to_string(),
            });
        }
    };

    let handshake_ack = match conn.socket.next().await {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => {
                if let Msg::HandshakeAck(h) = msg {
                    h
                } else {
                    return Err(HandshakeInitError::HandshakeAckWrongArrived);
                }
            }
            Err(err) => {
                return Err(HandshakeInitError::HandshakeAckParseFail {
                    err: err.to_string(),
                });
            }
        },
        None => {
            return Err(HandshakeInitError::HandshakeAckNotYetArrived);
        }
    };

    let her_public_key_str = handshake_ack.src_public_key_str;
    let my_secret_key = &identity.credential.secret_key;
    let her_public_key =
        match sak_crypto::convert_public_key_str_into_public_key(&her_public_key_str) {
            Ok(pk) => pk,
            Err(err) => {
                return Err(HandshakeInitError::PublicKeyCreateFail {
                    public_key: her_public_key_str.clone(),
                    err,
                })
            }
        };

    let shared_secret = sak_crypto::make_shared_secret(my_secret_key, her_public_key);

    let upgraded_conn = match conn
        .upgrade(shared_secret, &[0; 12], &her_public_key_str)
        .await
    {
        Ok(c) => c,
        Err(err) => {
            return Err(HandshakeInitError::ConnectionCreateFail {
                err: err.to_string(),
            });
        }
    };

    let transport = Transport {
        conn: RwLock::new(upgraded_conn),
    };

    return Ok(transport);
}
