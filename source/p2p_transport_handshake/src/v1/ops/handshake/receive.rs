use super::Handshake;
use chrono::{DateTime, Duration, Utc};
use colored::Colorize;
use logger::tdebug;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::{Peer, PeerSlot, PeerStatus, PeerTable};
use p2p_transport::{connection::Connection, transport::Transport};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::RwLock;

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

    #[error("Cannot acquire an empty slot, err: {err}")]
    EmptySlotNotAvailable { err: String },

    #[error("handshake has been done with this peer lately")]
    HandshakeRecentlySucceeded,
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
    println!("receive_handshake()");

    let HandshakeRecvArgs {
        my_p2p_port,
        handshake_syn,
        p2p_identity,
        p2p_peer_table,
        ..
    } = handshake_recv_args;

    let Handshake {
        instance_id,
        src_p2p_port,
        src_public_key_str: her_public_key_str,
        dst_public_key_str: my_public_key_str,
    } = handshake_syn;

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

    let peer_slot = match p2p_peer_table
        .get_mapped_peer_lock(&her_public_key_str)
        .await
    {
        Some(p) => {
            if let PeerStatus::HandshakeSuccess { at } = p.status {
                let now = Utc::now();
                if now.signed_duration_since(at) < Duration::seconds(60) {
                    return Err(HandshakeRecvError::HandshakeRecentlySucceeded);
                }
            }
            PeerSlot::Peer(p)
        }
        None => match p2p_peer_table.get_empty_slot().await {
            Ok(s) => PeerSlot::Slot(s),
            Err(err) => {
                return Err(HandshakeRecvError::EmptySlotNotAvailable { err });
            }
        },
    };

    let shared_secret =
        crypto::make_shared_secret(my_secret_key, her_public_key);

    let handshake_ack = Handshake {
        instance_id: instance_id.clone(),
        src_p2p_port: my_p2p_port,
        src_public_key_str: my_public_key_str.clone(),
        dst_public_key_str: her_public_key_str.clone(),
    };

    let handshake_ack_frame = handshake_ack.into_ack_frame();

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
        shared_secret,
    };

    match peer_slot {
        PeerSlot::Slot(s) => {
            let p = Peer {
                transport,
                p2p_port: src_p2p_port,
                public_key_str: her_public_key_str.clone(),
                addr_guard: None,
                status: PeerStatus::HandshakeSuccess { at: Utc::now() },
                __internal_slot_guard: s,
            };

            let peer = Arc::new(RwLock::new(p));

            p2p_peer_table
                .insert_mapping(&her_public_key_str, peer)
                .await;
        }
        PeerSlot::Peer(mut peer) => {
            peer.transport = transport;
            peer.p2p_port = src_p2p_port;
            peer.public_key_str = her_public_key_str.clone();
            peer.status = PeerStatus::HandshakeSuccess { at: Utc::now() };
        }
    };

    tdebug!(
        "p2p_trpt_hske",
        "receive",
        "Peer node updated, hs_id: {}, her_public_key: {}, \
            addr_guard None",
        &instance_id,
        her_public_key_str.clone().green(),
    );

    Ok(())
}
