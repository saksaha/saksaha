use crate::AddValidatorParams;
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, ContractError, CtrRequest,
    RequestArgs, Storage,
};
// use sak_store_accessor::StoreAccessor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PUB_KEY_LEN: usize = 130;

const VALIDATOR_LEN: usize = PUB_KEY_LEN;

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorStorage {
    pub validators: Vec<String>,
}

#[link(wasm_import_module = "host")]
extern "C" {
    fn hello(param1: i32, param2: i32) -> i32;

    fn HOST__get_mrs_data(param1: *mut u8, param2: i32) -> i32;

    fn get_latest_len(p1: i32, p2: i32) -> i32;
}

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Vec<u8>, ContractError> {
    let storage = ValidatorStorage {
        validators: vec![
            // TODO public_key of 'dev_local_1' profile
            // This should be inserted via a separate tx
            "045739d074b8722891c307e8e75c9607e0b55a80778\
                b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
                e95590616fd382662a09653f2a966ff524989ae8c0f"
                .to_string(),
        ],
    };

    let v = serde_json::to_vec(&storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: Storage,
) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        "get_validator" => {
            return handle_get_validator(storage);
        }
        _ => {
            return Err(format!("Wrong request type has been found").into());
        }
    };
}

fn handle_get_validator(storage: Storage) -> Result<Vec<u8>, ContractError> {
    let validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

    let validator = validator_storage
        .validators
        .get(0)
        .ok_or(format!("Validators are empty"))?;

    let ret = validator.to_owned().into_bytes();

    Ok(ret)
}

define_execute!();
pub fn execute2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: &mut Storage,
) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        "add_validator" => {
            return handle_add_validator(storage, request.args);
        }
        _ => {
            return Err(format!("Wrong request type has been found").into());
        }
    };
}

fn handle_add_validator(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<Vec<u8>, ContractError> {
    let mut validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

    let add_validator_params: AddValidatorParams = serde_json::from_slice(&args)?;

    validator_storage
        .validators
        .push(add_validator_params.validator);

    *storage = serde_json::to_vec(&validator_storage)?;

    Ok(vec![])
}
