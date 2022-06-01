use colored::Colorize;
use futures::SinkExt;
use futures::StreamExt;
use logger::{tdebug, twarn};
use p2p_addr::KnownAddr;
use p2p_discovery::Addr;
use p2p_identity::Identity;
use p2p_peer_table::SlotGuard;
use p2p_peer_table::{Peer, PeerStatus, PeerTable};
use p2p_transport::Handshake;
use p2p_transport::Msg;
use p2p_transport::{Connection, Transport};
use std::sync::Arc;
use thiserror::Error;
use tokio::net::TcpStream;
use tokio::sync::{OwnedRwLockWriteGuard, RwLock};

pub struct HandshakeInitArgs {
    pub peer_table: Arc<PeerTable>,
    pub identity: Arc<Identity>,
    pub addr: Arc<Addr>,
}

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

    #[error("Handshake is not fully arrived")]
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
}

pub async fn initiate_handshake(
    handshake_init_args: HandshakeInitArgs,
) -> Result<(), HandshakeInitError> {
    let HandshakeInitArgs {
        identity,
        peer_table,
        addr,
    } = handshake_init_args;

    let known_addr = &addr.known_addr;

    let peer_slot_guard =
        match peer_table.get_mapped_peer(&known_addr.public_key_str).await {
            Some(_) => {
                return Err(HandshakeInitError::PeerAlreadyMapped);
            }
            None => match peer_table.get_empty_slot().await {
                Ok(s) => s,
                Err(_) => {
                    return Err(HandshakeInitError::EmptyNodeNotAvailable);
                }
            },
        };

    let endpoint = known_addr.p2p_endpoint();

    if utils_net::is_my_endpoint(identity.p2p_port, &endpoint) {
        twarn!(
            "saksaha",
            "p2p",
            "Cannot make a request to myself, abandoning handshake \
                    init task, endopint: {}",
            &endpoint
        );

        return Err(HandshakeInitError::MyEndpoint {
            p2p_endpoint: endpoint,
        });
    }

    let mut conn = match TcpStream::connect(&endpoint).await {
        Ok(s) => {
            let c = match Connection::new(s) {
                Ok(c) => c,
                Err(err) => {
                    return Err(HandshakeInitError::ConnectionFail {
                        err: err.to_string(),
                    });
                }
            };

            tdebug!(
                "saksaha",
                "p2p",
                "(caller) TCP connected to destination, \
                        peer_addr: {:?}",
                c.socket_addr,
            );

            c
        }
        Err(err) => {
            return Err(HandshakeInitError::ConnectionFail {
                err: err.to_string(),
            });
        }
    };

    let handshake = match Handshake::new(
        identity.p2p_port,
        identity.credential.public_key_str.clone(),
        known_addr.public_key_str.clone(),
    ) {
        Ok(h) => h,
        Err(err) => {
            return Err(HandshakeInitError::HandshakeMsgInitFail { err });
        }
    };

    match conn.socket_tx.send(Msg::HandshakeSyn(handshake)).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeInitError::FrameWriteFail {
                err: err.to_string(),
            });
        }
    };

    let handshake_ack = match conn.socket_rx.next().await {
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
        None => return Err(HandshakeInitError::HandshakeAckNotYetArrived),
    };

    let her_public_key_str = handshake_ack.src_public_key_str;
    let my_secret_key = &identity.credential.secret_key;
    let her_public_key = match crypto::convert_public_key_str_into_public_key(
        &her_public_key_str,
    ) {
        Ok(pk) => pk,
        Err(err) => {
            return Err(HandshakeInitError::PublicKeyCreateFail {
                public_key: her_public_key_str.clone(),
                err,
            })
        }
    };

    {
        let shared_secret =
            crypto::make_shared_secret(my_secret_key, her_public_key);

        let transport = Transport {
            conn: RwLock::new(conn),
            shared_secret,
        };

        let peer = {
            let p = Peer {
                p2p_port: handshake_ack.src_p2p_port,
                public_key_str: her_public_key_str.clone(),
                addr,
                transport,
                status: PeerStatus::HandshakeInit,
                peer_slot_guard,
            };

            Arc::new(p)
        };

        peer_table.insert_mapping(peer).await;
    }

    Ok(())
}
