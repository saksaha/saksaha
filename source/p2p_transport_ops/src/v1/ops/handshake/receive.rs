use chrono::Utc;
use colored::Colorize;
use futures::SinkExt;
use logger::tdebug;
use p2p_discovery::Addr;
use p2p_identity::Identity;
use p2p_peer_table::{Peer, PeerStatus, PeerTable};
use p2p_transport::{Connection, Handshake, Msg, Transport};
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{OwnedRwLockWriteGuard, RwLock};

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

    #[error("Cannot insert handshaked peer into the map, err: {err}")]
    InsertMappingFail { err: String },

    #[error("handshake has been done with this peer lately")]
    HandshakeRecentlySucceeded,
}

pub struct HandshakeRecvArgs {
    pub handshake_syn: Handshake,
    pub identity: Arc<Identity>,
    pub peer_table: Arc<PeerTable>,
    pub addr: Arc<Addr>,
}

pub async fn receive_handshake(
    handshake_recv_args: HandshakeRecvArgs,
    mut conn: Connection,
) -> Result<(), HandshakeRecvError> {
    let HandshakeRecvArgs {
        handshake_syn,
        peer_table,
        identity,
        addr,
    } = handshake_recv_args;

    let Handshake {
        instance_id,
        src_p2p_port,
        src_public_key_str: her_public_key_str,
        dst_public_key_str: my_public_key_str,
    } = handshake_syn;

    if my_public_key_str != identity.credential.public_key_str {
        return Err(HandshakeRecvError::UnmatchedMyPublicKey {
            public_key: my_public_key_str,
        });
    }

    let my_secret_key = &identity.credential.secret_key;
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

    let slot_guard = match peer_table.get_empty_slot().await {
        Ok(s) => s,
        Err(err) => {
            return Err(HandshakeRecvError::EmptySlotNotAvailable { err });
        }
    };

    let shared_secret =
        crypto::make_shared_secret(my_secret_key, her_public_key);

    let handshake = Handshake {
        instance_id: instance_id.clone(),
        src_p2p_port: identity.p2p_port,
        src_public_key_str: my_public_key_str.clone(),
        dst_public_key_str: her_public_key_str.clone(),
    };

    match conn.socket_tx.send(Msg::HandshakeAck(handshake)).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeRecvError::AckSendFail {
                err: err.to_string(),
            });
        }
    };

    {
        let transport = Transport {
            conn: RwLock::new(conn),
            shared_secret,
        };

        let peer = {
            let p = Peer {
                transport,
                p2p_port: src_p2p_port,
                public_key_str: her_public_key_str.clone(),
                addr,
                status: PeerStatus::HandshakeSuccess { at: Utc::now() },
                peer_slot_guard: slot_guard,
            };

            Arc::new(p)
        };

        if let Err(err) = peer_table.insert_mapping(peer).await {
            return Err(HandshakeRecvError::InsertMappingFail { err });
        }
    }

    Ok(())
}
