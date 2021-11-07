use thiserror::Error;
use std::sync::Arc;
use log::{debug, error};
use saksaha_p2p_discovery::address::Address;
use saksaha_p2p_identity::PUBLIC_KEY_LEN;
use crate::p2p::state::HostState;

pub(crate) struct HandshakeInitiate {
    host_state: Arc<HostState>,
}

#[derive(Error, Debug)]
pub enum HandshakeInitError {
    #[error("Aborting, request to my endpoint: {endpoint}")]
    MyEndpoint { endpoint: String },
}

impl HandshakeInitiate {
    pub fn new(host_state: Arc<HostState>) -> HandshakeInitiate {
        HandshakeInitiate {
            host_state,
        }
    }

    pub async fn send_handshake_syn(
        &self,
        ip: String,
        p2p_port: u16,
        public_key: [u8; PUBLIC_KEY_LEN]
    ) -> Result<(), HandshakeInitError> {
        let endpoint = format!("{}:{}", ip, p2p_port);

        let my_p2p_port = self.host_state.my_p2p_port;

        if super::is_my_endpoint(my_p2p_port, &endpoint) {
            return Err(HandshakeInitError::MyEndpoint {
                endpoint,
            });
        }



        // let my_sig = self.disc_state.id.sig();
        // let my_public_key_bytes = self.disc_state.id.public_key_bytes();

        // let way_syn =
        //     WhoAreYouSyn::new(my_sig, my_p2p_port, my_public_key_bytes);

        // let buf = match way_syn.to_bytes() {
        //     Ok(b) => b,
        //     Err(err) => {
        //         return Err(WhoareyouInitError::ByteConversionFail { err });
        //     }
        // };

        // self.udp_socket.send_to(&buf, endpoint.clone()).await?;

        // debug!(
        //     "Successfully sent WhoAreYou to endpoint: {}, buf len: {}",
        //     &endpoint,
        //     buf.len()
        // );


        Ok(())
    }
}
