mod dial;
mod listen;

use crate::{common::SakResult, err_res};

pub struct PeerOp {}

impl PeerOp {
    pub fn new() -> SakResult<PeerOp> {
        let peer_op = PeerOp {};

        Ok(peer_op)
    }
}

impl PeerOp {
    pub async fn start(self) -> SakResult<bool> {
        let listen = listen::Listen {};

        tokio::spawn(async move {
            match listen.start_listening().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!(
                        "Error start peer op listening, err: {}",
                        err
                    );
                }
            }
        });

        let dial = dial::Dial {};

        tokio::spawn(async move {
            match dial.start_dialing().await {
                Ok(_) => Ok(()),
                Err(err) => {
                    return err_res!(
                        "Error start peer op dialing, err: {}",
                        err
                    );
                }
            }
        });

        Ok(true)
    }
}
