use crate::SlotGuard;
use chrono::{DateTime, Utc};
use p2p_discovery::Addr;
use p2p_transport::Transport;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Peer {
    pub p2p_port: u16,
    pub public_key_str: String,
    pub transport: Transport,
    pub status: PeerStatus,
    // pub addr: Arc<RwLock<Addr>>,
    pub addr: Arc<Addr>,
    pub peer_slot_guard: SlotGuard,
}

pub enum PeerStatus {
    Initialized,
    HandshakeSuccess { at: DateTime<Utc> },
    HandshakeInit,
    HandshakeInitFail { err: String },
    HandshakeRecvFail { err: String },
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let socket_addr = &self.transport.conn.socket_addr;
        let public_key_str = &self.public_key_str;

        write!(
            f,
            "Peer (public_key_str: {}, status: {})",
            public_key_str, self.status,
        )
    }
}

impl std::fmt::Display for PeerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            PeerStatus::Initialized => {
                write!(f, "Initialized")
            }
            PeerStatus::HandshakeInit => {
                write!(f, "Handshake initiated")
            }
            PeerStatus::HandshakeSuccess { at } => {
                write!(f, "HandshakeSuccess, at: {}", at)
            }
            PeerStatus::HandshakeInitFail { err } => {
                write!(f, "HandshakeInitFail, err: {}", err)
            }
            PeerStatus::HandshakeRecvFail { err } => {
                write!(f, "HandshakeRecvFail, err: {}", err)
            }
        }
    }
}
