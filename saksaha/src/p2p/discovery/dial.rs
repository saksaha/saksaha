use logger::log;
use crate::{common::SakResult, err_res};
use super::Disc;

pub struct Dial {
    pub bootstrap_peers: Option<Vec<String>>,
}

impl Dial {
    pub async fn start_dialing(&self) -> SakResult<bool> {
        log!(DEBUG, "Start disc dialing\n");

        Ok(true)
    }
}
