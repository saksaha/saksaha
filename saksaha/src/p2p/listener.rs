use crate::{
    peer::peer_store::PeerStore,
};
use log::{debug, info};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Listener {
    identity: Arc<Identity>,
    peer_store: Arc<PeerStore>,
    rpc_port: u16,
    tcp_listener: TcpListener,
}

impl Listener {
    pub fn new(
        identity: Arc<Identity>,
        peer_store: Arc<PeerStore>,
        tcp_listener: TcpListener,
        rpc_port: u16,
    ) -> Listener {
        Listener {
            identity,
            peer_store,
            rpc_port,
            tcp_listener,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        Ok(())
    }
}
