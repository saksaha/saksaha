use logger::log;
use crate::{common::SakResult, err_res};
use super::Disc;

impl Disc {
    pub async fn start_dialing(&self) {
        log!(DEBUG, "Start discovery dialing\n");
    }
}
