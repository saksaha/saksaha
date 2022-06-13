use crate::SlotGuard;
use chrono::{DateTime, Utc};
use sak_p2p_disc::DiscAddr;
use sak_p2p_trpt::Transport;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Peer {
    pub p2p_port: u16,
    pub public_key_str: String,
    pub transport: Transport,
    pub status: RwLock<PeerStatus>,
    pub addr: Arc<DiscAddr>,
    pub peer_slot_guard: SlotGuard,
}

pub enum PeerStatus {
    Initialized,
    HandshakeSuccess { at: DateTime<Utc> },
    HandshakeInit,
    HandshakeInitFail { err: String },
    HandshakeRecvFail { err: String },
    Disconnected,
}

impl Peer {
    pub fn public_key_short(&self) -> &str {
        &self.public_key_str[..6]
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match &self.status.try_read() {
            Ok(s) => s.to_string(),
            Err(_) => "being used".to_string(),
        };

        write!(
            f,
            "Peer (ip: {}, public_key_str: {}, status: {})",
            &self.addr.known_addr.ip, &self.public_key_str, status,
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
            PeerStatus::Disconnected => {
                write!(f, "Disconnected",)
            }
        }
    }
}
