mod dial;
mod listen;

use crate::{common::SakResult, err_res};

#[derive(Clone, Copy)]
pub struct PeerOp {}

impl PeerOp {
    pub fn new() -> SakResult<PeerOp> {
        let peer_op = PeerOp {};

        Ok(peer_op)
    }
}

impl PeerOp {
    pub async fn start(self) -> SakResult<bool> {
        tokio::spawn(async move {
            if let Err(err) = self.start_dialing().await {

            }
        });

        tokio::spawn(async move {
            if let Err(err) = self.start_listening().await {

            }
        });

        Ok(true)
    }
}
