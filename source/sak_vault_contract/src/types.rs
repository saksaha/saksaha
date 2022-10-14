use sak_contract_std::ContractError;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type VaultKey = Vec<u8>;
pub type VaultValue = Vec<u8>;

pub mod request_type {
    pub const GET_VALUE: &'static str = "get_value";

    pub const PUT_VALUE: &'static str = "put_value";

    pub const PUT_KEY_SPEC: &'static str = "put_key_spec";
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VaultStorage {
    pub vault: HashMap<VaultKey, VaultValue>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetValueParams {
    pub vault_key: VaultKey,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PutValueParams {
    pub vault_key: VaultKey,
    pub vault_value: VaultValue,
}
