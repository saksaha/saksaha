use logger::log;
use crate::{common::SakResult, err_res};
use super::PeerOp;

pub struct Dial {}

impl Dial {
    pub async fn start_dialing(self) -> SakResult<bool> {
        log!(DEBUG, "start p2p dialing\n");

        Ok(true)
    }
}
