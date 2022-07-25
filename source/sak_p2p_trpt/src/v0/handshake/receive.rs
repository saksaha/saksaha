use crate::TrptError;
use crate::{Connection, Handshake, Msg, Transport};
use futures::SinkExt;
use futures::StreamExt;
use log::warn;
use sak_p2p_id::Identity;
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum HandshakeRecvError {
    #[error("Msg of this type is not expected at this stage.")]
    InvalidMsgType,

    #[error("Could not parse the message, err: {err}")]
    MsgParseError { err: TrptError },

    #[error("Peer has ended the connection, socket_addr: {socket_addr}")]
    PeerEndedConnection { socket_addr: SocketAddr },

    #[error(
        "She does not know my public key correct, my public key (she knows)\
        : {public_key}"
    )]
    UnmatchedMyPublicKey { public_key: String },

    #[error(
        "Cannot create public key from the public key string given, \
        public key: {public_key}, err: {err}"
    )]
    PublicKeyCreateFail { public_key: String, err: String },

    #[error("Failed to send handshake ack frame, err: {err}")]
    AckSendFail { err: String },

    #[error(
        "Peer node is being used by another process (task), \
        public_key: {public_key}, err: {err}"
    )]
    PeerNodeAlreadyInUse { public_key: String, err: String },

    #[error("Peer node is invalid. Its value not being peer")]
    PeerNodeNotHavingPeer,

    #[error("Cannot acquire an empty slot, err: {err}")]
    EmptySlotNotAvailable { err: String },

    #[error("Cannot insert handshaked peer into the map, err: {err}")]
    InsertMappingFail { err: String },

    #[error("handshake has been done with this peer lately")]
    HandshakeRecentlySucceeded,
}

pub struct HandshakeRecvArgs {
    // pub handshake_syn: Handshake,
    pub identity: Arc<Identity>,
}

pub async fn receive_handshake(
    handshake_recv_args: HandshakeRecvArgs,
    mut conn: Connection,
) -> Result<(Transport, String), HandshakeRecvError> {
    let HandshakeRecvArgs {
        // handshake_syn,
        identity,
    } = handshake_recv_args;

    let handshake_syn = match conn.socket.next().await {
        Some(maybe_msg) => match maybe_msg {
            Ok(msg) => match msg {
                Msg::HandshakeSyn(handshake) => handshake,
                _ => {
                    return Err(HandshakeRecvError::InvalidMsgType);
                }
            },
            Err(err) => {
                return Err(HandshakeRecvError::MsgParseError { err });
            }
        },
        None => {
            return Err(HandshakeRecvError::PeerEndedConnection {
                socket_addr: conn.socket_addr,
            });
        }
    };

    let Handshake {
        instance_id,
        src_p2p_port: _,
        src_public_key_str: her_public_key_str,
        dst_public_key_str: my_public_key_str,
    } = handshake_syn;

    if my_public_key_str != identity.credential.public_key_str {
        return Err(HandshakeRecvError::UnmatchedMyPublicKey {
            public_key: my_public_key_str,
        });
    }

    let handshake = Handshake {
        instance_id: instance_id.clone(),
        src_p2p_port: identity.p2p_port,
        src_public_key_str: my_public_key_str.clone(),
        dst_public_key_str: her_public_key_str.clone(),
    };

    match conn.socket.send(Msg::HandshakeAck(handshake)).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeRecvError::AckSendFail {
                err: err.to_string(),
            });
        }
    };

    let my_secret_key = &identity.credential.secret_key;

    let her_public_key =
        match sak_crypto::convert_public_key_str_into_public_key(
            &her_public_key_str,
        ) {
            Ok(pk) => pk,
            Err(err) => {
                return Err(HandshakeRecvError::PublicKeyCreateFail {
                    public_key: her_public_key_str,
                    err,
                })
            }
        };

    let shared_secret =
        sak_crypto::make_shared_secret(my_secret_key, her_public_key);

    let upgraded_conn = conn.upgrade(shared_secret, &[0; 12]);

    let transport = Transport {
        conn: RwLock::new(upgraded_conn),
    };

    return Ok((transport, her_public_key_str));
}
