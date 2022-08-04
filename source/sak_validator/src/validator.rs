use crate::AddValidatorParams;
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, Request, RequestArgs, Storage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const PUB_KEY_LEN: usize = 130;

const VALIDATOR_LEN: usize = PUB_KEY_LEN;

#[derive(Serialize, Deserialize, Debug)]
pub struct ValidatorStorage {
    pub validators: Vec<String>,
}

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    let storage = ValidatorStorage { validators: vec![] };

    let v = serde_json::to_vec(&storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    request: Request,
    storage: Storage,
) -> Result<String, ContractError> {
    return Ok("apo.".to_string());
    // match request.req_type.as_ref() {
    //     "get_validator" => {
    //         return handle_get_validator(storage);
    //     }
    //     _ => {
    //         return Err(ContractError {
    //             err_msg: "Wrong request type has been found".to_string(),
    //         });
    //     }
    // };
}

fn handle_get_validator(storage: Storage) -> Result<String, ContractError> {
    let validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

    let validator = validator_storage
        .validators
        .get(0)
        .ok_or(format!("Validators are empty"))?
        .to_string();

    Ok(validator)
}

define_execute!();
pub fn execute2(
    storage: &mut Storage,
    request: Request,
) -> Result<(), ContractError> {
    match request.req_type.as_ref() {
        "add_validator" => {
            return handle_add_validator(storage, request.args);
        }
        _ => {
            return Err(ContractError {
                err_msg: "Wrong request type has been found".to_string(),
            });
        }
    };
}

fn handle_add_validator(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let mut validator_storage: ValidatorStorage =
        serde_json::from_slice(&storage)?;

    let add_validator_params: AddValidatorParams =
        serde_json::from_slice(&args)?;

    validator_storage
        .validators
        .push(add_validator_params.validator);

    *storage = serde_json::to_vec(&validator_storage)?;

    Ok(())
}
