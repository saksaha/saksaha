use crate::{get_mrs_data_from_host, RET_LEN_SIZE};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug)]
pub struct List {
    _name: String,
    data: HashMap<String, String>,
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

    pub fn push(&self, value: &String) -> Vec<u8> {
        //TO-DO: get latest idx of the stored List

        let latest_idx = 10;

        let key: String = format!("{}_{}", self._name, latest_idx);

        // push_mrs_data_to_host(&key, value);

        vec![0]
    }

    pub fn receipt(&self) -> HashMap<String, Vec<u8>> {
        HashMap::from([("str_2".to_string(), vec![234])])
    }
}
