mod dial;

mod listen;

use crate::{common::SakResult, err_res};
use listen::{Listen};
use dial::{Dial};

pub struct PeerOp {
    pub listen: listen::Listen,
    pub dial: dial::Dial,
}

impl PeerOp {
    pub fn new() -> SakResult<PeerOp> {
        let listen = Listen{};
        let dial = Dial{};
        let peer_op = PeerOp {
            listen,
            dial,
        };

        Ok(peer_op)
    }
}

impl PeerOp {
    pub async fn start(&self) -> SakResult<bool> {
        Ok(true)
    }
}
