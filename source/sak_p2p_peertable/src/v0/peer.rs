use crate::SlotGuard;
use chrono::{DateTime, Utc};
use sak_p2p_addr::AddrStatus;
use sak_p2p_discovery::DiscAddr;
use sak_p2p_transport::Transport;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Peer {
    transport: Transport,
    peer_status: RwLock<PeerStatus>,
    addr: Arc<DiscAddr>,
    peer_slot_guard: SlotGuard,
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
    pub fn new(
        transport: Transport,
        peer_status: RwLock<PeerStatus>,
        addr: Arc<DiscAddr>,
        peer_slot_guard: SlotGuard,
    ) -> Peer {
        Peer {
            transport,
            peer_status,
            addr,
            peer_slot_guard,
        }
    }

    pub fn get_transport(&self) -> &Transport {
        &self.transport
    }

    pub fn get_public_key_short(&self) -> &str {
        &self.addr.known_addr.public_key_str[..6]
    }

    pub fn get_public_key(&self) -> &str {
        &self.addr.known_addr.public_key_str
    }

    pub fn get_peer_status(&self) -> &RwLock<PeerStatus> {
        &self.peer_status
    }

    pub fn get_addr(&self) -> &Arc<DiscAddr> {
        &self.addr
    }

    pub async fn set_peer_status(&self, peer_status: PeerStatus) {
        match &peer_status {
            PeerStatus::Disconnected => {
                let mut addr_status_lock =
                    self.addr.known_addr.status.write().await;

                *addr_status_lock = AddrStatus::Disconnected;
            }
            _ => (),
        }

        let mut peer_status_lock = self.peer_status.write().await;
        *peer_status_lock = peer_status;
    }
}

impl std::fmt::Display for Peer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let status = match &self.peer_status.try_read() {
            Ok(s) => s.to_string(),
            Err(_) => "being used".to_string(),
        };

        write!(
            f,
            "Peer (ip: {}, public_key_str: {}, status: {})",
            &self.addr.known_addr.ip,
            &self.get_public_key(),
            status,
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
