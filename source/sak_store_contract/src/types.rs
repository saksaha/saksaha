use sak_contract_std::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Key = String;

pub type StoreValue = String;

pub mod request_type {
    pub const GET_VALUE: &'static str = "get_value";

    pub const PUT_VALUE: &'static str = "put_value";

    pub const PUT_KEY_SPEC: &'static str = "put_key_spec";
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StoreKey {
    MRS(Key),
    LEDGER(Key),
}

impl StoreKey {
    pub fn get_key(&self) -> String {
        match self {
            StoreKey::MRS(s) => s.to_owned(),
            StoreKey::LEDGER(s) => s.to_owned(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StoreStorage {
    pub store: HashMap<Key, StoreValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetValueParams {
    pub store_key: StoreKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutValueParams {
    pub store_key: StoreKey,
    pub store_value: StoreValue,
}
