use crate::v1::initiate::TransportInitError;
use super::initiate;
use log::{debug, error};
use p2p_active_calls::ActiveCalls;
use p2p_identity::{Identity, PeerId};
use peer::Peer;
use std::sync::Arc;
use thiserror::Error;

pub(crate) struct TransportMeta {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
    pub active_calls: Arc<ActiveCalls>,
}

pub struct TransportFactory {
    transport_meta: Arc<TransportMeta>,
}

impl TransportFactory {
    pub fn new(
        identity: Arc<Identity>,
        my_rpc_port: u16,
        my_p2p_port: u16,
    ) -> TransportFactory {
        let active_calls = {
            let c = ActiveCalls::new();
            Arc::new(c)
        };

        let transport_meta = {
            let m = TransportMeta {
                identity,
                my_rpc_port,
                my_p2p_port,
                active_calls,
            };
            Arc::new(m)
        };

        TransportFactory { transport_meta }
    }

    pub async fn initiate_handshake(
        &self,
        ip: String,
        p2p_port: u16,
        peer_id: PeerId,
        peer: Arc<Peer>,
    ) -> Result<(), TransportInitError> {
        let transport_meta = self.transport_meta.clone();

        let handshake_sent =
            initiate::send_handshake_syn(ip, p2p_port, transport_meta).await?;

        Ok(())
    }
}

pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
