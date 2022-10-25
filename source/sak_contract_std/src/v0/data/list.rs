use crate::{get_mrs_data_from_host, ContractError, RET_LEN_SIZE};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug)]
pub struct List {
    _name: String,
    data: HashMap<String, Vec<u8>>,
}

impl List {
    pub fn new(_name: String) -> List {
        List {
            _name,
            data: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }

    pub fn push(&mut self, value: Vec<u8>) {
        //TO-DO: get latest idx of the stored List and update index
        let latest_idx_key = String::from("latest_idx");
        let latest_idx = get_mrs_data_from_host(&latest_idx_key);

        let latest_idx = 0;

        let key: String = format!("{}_{}", self._name, latest_idx);

        self.data.insert(key, value);
    }

    pub fn get_receipt(&self) -> Result<Vec<u8>, ContractError> {
        let ret = serde_json::to_vec(&self.data)?;

        Ok(ret)
    }
}
