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

    pub fn push(&mut self, key: String, value: Vec<u8>) {
        //TO-DO: get latest idx of the stored List
        let latest_idx = 0;

        self.data.insert(key, value);
    }

    pub fn receipt(&self) -> Result<Vec<u8>, ContractError> {
        let ret = serde_json::to_vec(&self.data)?;

        Ok(ret)
    }
}
