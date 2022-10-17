use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Default)]
pub struct ValidatorStorage {
    // data: BTreeMap<Vec<u8>, Vec<u8>>,
    validators: Vec<String>,
}

impl ValidatorStorage {
    pub fn new() -> Self {
        ValidatorStorage::default()
    }

    pub fn add(&mut self, field_name: &str, default_value: &[u8]) {
        self.data
            .insert(field_name.as_bytes().to_vec(), default_value.to_vec());
    }
}
