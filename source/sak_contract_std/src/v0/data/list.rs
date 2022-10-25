use crate::{get_mrs_data_from_host, put_mrs_data_to_host, RET_LEN_SIZE};
use std::{collections::HashMap, convert::TryInto};

#[derive(Debug)]
pub struct List {
    _name: String,
    receipt: HashMap<String, Vec<u8>>,
}

impl List {
    pub fn new(_name: String) -> List {
        List {
            _name,
            receipt: HashMap::new(),
        }
    }

    pub fn get(&self, key: &String) -> Vec<u8> {
        let key: String = format!("{}_{}", self._name, key);

        let data = get_mrs_data_from_host(&key);

        data
    }

    pub fn push(&self, value: &String) {
        //TO-DO: get latest idx of the stored List
        let key: String = format!("{}", self._name);

        put_mrs_data_to_host(&key, value);
    }

    pub fn receipt(&self) -> HashMap<String, Vec<u8>> {
        HashMap::from([("str_2".to_string(), vec![234])])
    }
}
