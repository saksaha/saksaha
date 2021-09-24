use logger::log;
use crate::{common::SakResult, err_res};

pub struct Dial;

impl Dial {
    pub fn new() -> SakResult<Dial> {
        let dial = Dial {};

        Ok(dial)
    }
}

impl Dial {
    pub fn start(&self) -> SakResult<bool> {
        Ok(true)
    }
}
