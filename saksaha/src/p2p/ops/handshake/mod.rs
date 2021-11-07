use std::sync::Arc;

use crate::p2p::state::HostState;

use self::{initiate::HandshakeInitiate, receive::HandshakeReceive};

pub mod initiate;
pub mod receive;

pub(crate) struct HandshakeOp {
    pub initiate: HandshakeInitiate,
    pub receive: HandshakeReceive,
}

impl HandshakeOp {
    pub fn new(
        host_state: Arc<HostState>,
    ) -> HandshakeOp {
        let initiate = HandshakeInitiate::new(host_state.clone());

        let receive = HandshakeReceive::new(host_state.clone());

        HandshakeOp {
            initiate,
            receive,
        }
    }
}

fn is_my_endpoint(my_p2p_port: u16, endpoint: &String) -> bool {
    false
}
