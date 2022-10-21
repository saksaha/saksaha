use std::convert::TryInto;

use crate::{get_mrs_data_from_host, RET_LEN_SIZE};

crate::define_host_ffi!();
// crate::contract_bootstrap!();

#[derive(Debug)]
pub struct List {
    _name: String,
}

impl List {
    pub fn new(_name: String) -> List {
        List { _name }
    }

    pub fn receipt(&self) {}

    pub fn get(&self, key: &String) -> Vec<u8> {
        vec![0]
    }
}
