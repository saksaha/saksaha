use std::convert::TryInto;

use crate::{get_mrs_data_from_host, RET_LEN_SIZE};

crate::define_host_ffi!();
// crate::contract_bootstrap!();

#[derive(Debug)]
pub struct Dict {
    _name: String,
}

impl Dict {
    pub fn new(_name: String) -> Dict {
        Dict { _name }
    }

    pub fn receipt(&self) {}

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }
}
