mod dial;
mod listen;

use tokio::task::JoinHandle;

use crate::{common::SakResult, err_res};

pub struct PeerOp {
    // pub listen: listen::Listen,
// pub dial: dial::Dial,
}

impl PeerOp {
    pub fn new() -> SakResult<PeerOp> {
        let peer_op = PeerOp {};

        Ok(peer_op)
    }
}

impl PeerOp {
    pub async fn start(self) -> JoinHandle<(SakResult<bool>, SakResult<bool>)> {
        let handle = tokio::spawn(async move {
            tokio::join!(self.start_dialing(), self.start_listening())
        });

        handle
    }
}
