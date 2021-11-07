use crate::peer::peer_store::PeerStore;
use log::{debug, info};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::net::TcpListener;

use super::state::HostState;

pub(crate) struct Listener {
    tcp_listener: TcpListener,
    host_state: Arc<HostState>,
}

impl Listener {
    pub fn new(
        tcp_listener: TcpListener,
        host_state: Arc<HostState>,
    ) -> Listener {
        Listener {
            tcp_listener,
            host_state,
        }
    }

    pub async fn start(&self) -> Result<(), String> {
        Ok(())
    }
}
