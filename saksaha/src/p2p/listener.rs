use crate::{p2p::credential::Credential, peer::peer_store::PeerStore};
use log::{debug, info};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{mpsc::Receiver, Mutex},
};

pub struct Listener {
    credential: Arc<Box<dyn Identity + Send + Sync>>,
    peer_store: Arc<PeerStore>,
    tcp_listener: TcpListener,

    rpc_port: u16,
}

impl Listener {
    pub fn new(
        credential: Arc<Box<dyn Identity + Send + Sync>>,
        peer_store: Arc<PeerStore>,
        tcp_listener: TcpListener,
        rpc_port: u16,
    ) -> Listener {
        Listener {
            credential,
            peer_store,
            tcp_listener,
            rpc_port,
        }
    }

    pub async fn start(
        &self,
    ) -> Result<(), String> {
        Ok(())
    }
}
