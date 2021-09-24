mod dial;
mod listen;

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

struct R<'inner, T> {
    inner: &'inner T,
}

impl PeerOp {
    pub async fn start(self) -> SakResult<bool> {
        // let mut a = std::sync::Arc::new(self);
        // let mut b = a.clone();

        // let b = self.clone();

        tokio::spawn(async move {
            let _ = tokio::join!(self.start_dialing(), self.start_listening());
        });

        Ok(true)
    }
}
