use super::initiate;
use log::{debug, error};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use thiserror::Error;

pub(crate) struct TransportMeta {
    pub identity: Arc<Identity>,
    pub my_rpc_port: u16,
    pub my_p2p_port: u16,
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
        let transport_meta = {
            let m = TransportMeta {
                identity,
                my_rpc_port,
                my_p2p_port,
            };
            Arc::new(m)
        };

        TransportFactory { transport_meta }
    }

    pub async fn initiate_handshake(
        &self,
        ip: String,
        p2p_port: u16,
    ) -> Result<(), String> {
        initiate::initiate_handshake(self.transport_meta.clone(), ip, p2p_port)
            .await
    }
}

pub(crate) fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
