use super::Handshake;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::{NodeValue, Peer, PeerTable};
use p2p_transport::{connection::Connection, transport::Transport};
use std::{sync::Arc, time::Duration};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum HandshakeRecvError {
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

    #[error("Peer node (in table) reserve failed, err: {err}")]
    PeerNodeReserveFail { err: String },
}

pub struct HandshakeRecvArgs {
    pub handshake_syn: Handshake,
    pub my_p2p_port: u16,
    pub src_p2p_port: u16,
    pub p2p_identity: Arc<P2PIdentity>,
    pub p2p_peer_table: Arc<PeerTable>,
}

pub async fn receive_handshake(
    handshake_recv_args: HandshakeRecvArgs,
    mut conn: Connection,
) -> Result<(), HandshakeRecvError> {
    let HandshakeRecvArgs {
        my_p2p_port,
        handshake_syn,
        src_p2p_port,
        p2p_identity,
        p2p_peer_table,
    } = handshake_recv_args;

    let Handshake {
        src_p2p_port,
        src_public_key_str: her_public_key_str,
        dst_public_key_str: my_public_key_str,
    } = handshake_syn;

    println!("handshake recv, dst_public_key: {}", my_public_key_str);
    println!(
        "handshake recv, src_public_key: {}",
        p2p_identity.public_key_str
    );

    if my_public_key_str != p2p_identity.public_key_str {
        return Err(HandshakeRecvError::UnmatchedMyPublicKey {
            public_key: my_public_key_str,
        });
    }

    let my_secret_key = &p2p_identity.secret_key;
    let her_public_key = match crypto::convert_public_key_str_into_public_key(
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

    let peer_node_guard = match p2p_peer_table.get(&her_public_key_str).await {
        Some(n) => match n {
            Ok(n) => n,
            Err(err) => {
                return Err(HandshakeRecvError::PeerNodeAlreadyInUse {
                    public_key: her_public_key_str,
                    err,
                });
            }
        },
        None => match p2p_peer_table.reserve(&her_public_key_str).await {
            Ok(n) => n,
            Err(err) => {
                return Err(HandshakeRecvError::PeerNodeReserveFail { err });
            }
        },
    };

    let shared_secret =
        crypto::make_shared_secret(my_secret_key, her_public_key);

    let handshake_ack = Handshake {
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str.clone(),
        dst_public_key_str: her_public_key_str.clone(),
    };

    let handshake_ack_frame = handshake_ack.into_syn_frame();

    match conn.write_frame(&handshake_ack_frame).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeRecvError::AckSendFail {
                err: err.to_string(),
            });
        }
    };

    let transport = Transport {
        conn,
        p2p_port: src_p2p_port,
        public_key_str: her_public_key_str.clone(),
        shared_secret,
        addr_guard: None,
    };

    let mut peer_node_lock = peer_node_guard.node.lock().await;
    peer_node_lock.value = NodeValue::Valued(Peer { transport });

    Ok(())
}
