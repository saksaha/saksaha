use logger::log;
use crate::{common::SakResult, err_res};
use super::PeerOp;

impl PeerOp {
    pub async fn start_dialing(&self) -> SakResult<bool> {
        log!(DEBUG, "start p2p dialing\n");

        Ok(true)
    }
}
