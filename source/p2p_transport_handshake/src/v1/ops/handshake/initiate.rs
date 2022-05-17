use crate::ops::Handshake;
use colored::Colorize;
use logger::tdebug;
use p2p_discovery::{Addr, AddrGuard, AddrVal};
use p2p_identity::addr::KnownAddr;
use p2p_identity::identity::P2PIdentity;
use p2p_peer::{Peer, PeerSlot, PeerStatus, PeerTable};
use p2p_transport::connection::Connection;
use p2p_transport::parse::Parse;
use p2p_transport::transport::Transport;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{OwnedRwLockWriteGuard, RwLock};

pub struct HandshakeInitArgs {
    pub p2p_identity: Arc<P2PIdentity>,
    pub p2p_peer_table: Arc<PeerTable>,
    pub p2p_port: u16,
    pub addr_guard: AddrGuard,
    pub peer_slot: PeerSlot,
    pub addr_lock: OwnedRwLockWriteGuard<Addr>,
}

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("Could not intilize handshake msg, err: {err}")]
    HandshakeMsgInitFail { err: String },

    #[error("P2P Port may not be provided")]
    InvalidP2PEndpoint,

    #[error("Cannot send request to myself, addr: {addr}")]
    MyEndpoint { addr: KnownAddr },

    #[error("Cannot create tcp stream into endpoint, err: {err}")]
    ConnectionFail { err: String },

    #[error("Cannot retrieve peer address, err: {err}")]
    PeerAddressNotRetrievable { err: String },

    #[error("Cannot write frame (data) into connection, err: {err}")]
    FrameWriteFail { err: String },

    #[error("Data received may not be the entire frame intended, err: {err}")]
    InvalidFrame { err: String },

    #[error("Cannot read handshake ack msg, err: {err}")]
    HandshakeAckReadFail { err: String },

    #[error("Handshake ack frame received is not an array frame")]
    HandshakeAckNotArrayFrame,

    #[error(
        "Could not create a public key, err: {err}, \
         public_key: {public_key}"
    )]
    PublicKeyCreateFail { err: String, public_key: String },

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
    mut conn: Connection,
) -> Result<(), HandshakeInitError> {
    let HandshakeInitArgs {
        p2p_port,
        p2p_identity,
        addr_guard,
        p2p_peer_table,
        peer_slot,
        addr_lock,
    } = handshake_init_args;

    let known_addr = match &addr_lock.val {
        AddrVal::Known(k) => k,
        AddrVal::Unknown(_) => {
            return Err(HandshakeInitError::NotKnownAddr);
        }
    };

    let handshake_syn = match Handshake::new(
        p2p_port,
        p2p_identity.public_key_str.clone(),
        known_addr.public_key_str.clone(),
    ) {
        Ok(h) => h,
        Err(err) => {
            return Err(HandshakeInitError::HandshakeMsgInitFail { err });
        }
    };

    let handshake_syn_frame = handshake_syn.into_syn_frame();

    match conn.write_frame(&handshake_syn_frame).await {
        Ok(_) => (),
        Err(err) => {
            return Err(HandshakeInitError::FrameWriteFail {
                err: err.to_string(),
            });
        }
    };

    let handshake_ack_frame = match conn.read_frame().await {
        Ok(fr) => match fr {
            Some(f) => f,
            None => {
                return Err(HandshakeInitError::HandshakeAckReadFail {
                    err: "Peer might have closed the connection".into(),
                })
            }
        },
        Err(err) => {
            return Err(HandshakeInitError::HandshakeAckReadFail {
                err: err.to_string(),
            })
        }
    };

    let mut parse = match Parse::new(handshake_ack_frame) {
        Ok(p) => p,
        Err(_err) => {
            return Err(HandshakeInitError::HandshakeAckNotArrayFrame);
        }
    };

    let _frame_type = parse.next_string().unwrap();

    let handshake_ack = match Handshake::parse_frames(&mut parse) {
        Ok(h) => h,
        Err(err) => {
            return Err(HandshakeInitError::InvalidFrame {
                err: err.to_string(),
            });
        }
    };

    let her_public_key_str = handshake_ack.src_public_key_str;
    let my_secret_key = &p2p_identity.secret_key;
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

    let shared_secret =
        crypto::make_shared_secret(my_secret_key, her_public_key);

    let transport = Transport {
        conn,
        shared_secret,
    };

    match peer_slot {
        PeerSlot::Slot(s) => {
            let p = Peer {
                p2p_port: handshake_ack.src_p2p_port,
                public_key_str: her_public_key_str.clone(),
                addr_guard: Some(addr_guard),
                transport,
                status: PeerStatus::HandshakeInit,
                __internal_slot_guard: s,
            };

            let peer = Arc::new(RwLock::new(p));

            p2p_peer_table
                .insert_mapping(&her_public_key_str, peer)
                .await;
        }
        PeerSlot::Peer(mut peer) => {
            peer.p2p_port = handshake_ack.src_p2p_port;
            peer.public_key_str = her_public_key_str.clone();
            peer.addr_guard = Some(addr_guard);
            peer.transport = transport;
            peer.status = PeerStatus::HandshakeInit;
        }
    };

    tdebug!(
        "p2p_trpt_hske",
        "initiate",
        "Peer node updated, hs_id: {}, her_public_key: {}, \
            status: {:?}",
        &handshake_ack.instance_id,
        her_public_key_str.clone().green(),
        known_addr.status,
    );

    Ok(())
}
