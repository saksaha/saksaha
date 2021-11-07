use self::{initiate::HandshakeInitiate, receive::HandshakeReceive};

pub mod initiate;
pub mod receive;

pub struct HandshakeOp {
    pub initiate: HandshakeInitiate,
    pub receive: HandshakeReceive,
}

impl HandshakeOp {
    pub fn new() -> HandshakeOp {
        let initiate = HandshakeInitiate::new();
        let receive = HandshakeReceive::new();

        HandshakeOp {
            initiate,
            receive,
        }
    }
}
