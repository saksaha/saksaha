use crate::AddValidatorParams;
use sak_contract_derive::{CtrStateStore, MRSStore};
use sak_contract_std::{
    saksaha_contract, ContractError, CtrRequest, Dict, List, RequestArgs, Storage,
};

const PUB_KEY_LEN: usize = 130;

const VALIDATOR_LEN: usize = PUB_KEY_LEN;

saksaha_contract!(0.0.1);

#[derive(MRSStore, Debug)]
pub struct ValidatorMRSStorage {}

#[derive(CtrStateStore, Debug)]
pub struct ValidatorStorage {
    pub validators: List<String>,
}

pub fn init(ctx: &ContractCtx) -> Result<Vec<u8>, ContractError> {
    ctx.ctr_state.validators.put_bulk(["1".to_string()]);
    // let a = Vec::([1]);

    // let storage = ValidatorStorage {
    //     validators: vec![
    //         // TODO public_key of 'dev_local_1' profile
    //         // This should be inserted via a separate tx
    //         "045739d074b8722891c307e8e75c9607e0b55a80778\
    //             b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
    //             e95590616fd382662a09653f2a966ff524989ae8c0f"
    //             .to_string(),
    //     ],
    // };
    // let v = serde_json::to_vec(&storage)?;

    let v = vec![];

    Ok(v)
}

pub fn execute(ctx: &ContractCtx, request: CtrRequest) -> Result<Vec<u8>, ContractError> {
    // let storage = vec![];
    return Ok(vec![0]);

    // let storage = ValidatorStorage {
    //     validators: vec!["045739d074b8722891c307e8e75c9607e0b55a80778\
    //             b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
    //             e95590616fd382662a09653f2a966ff524989ae8c0f"
    //         .to_string()],
    // };

    // let storage = serde_json::to_vec(&storage)?;

    // match request.req_type.as_ref() {
    //     "get_validator" => {
    //         return handle_get_validator(storage);
    //     }
    //     _ => {
    //         return Err(format!("Wrong request type has been found").into());
    //     }
    // };
}

// fn handle_get_validator(storage: Storage) -> Result<Vec<u8>, ContractError> {
//     let validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

//     let validator = validator_storage
//         .validators
//         .get(0)
//         .ok_or(format!("Validators are empty"))?;

//     let ret = validator.to_owned().into_bytes();

//     // for wallet_web faucet test
//     // let ret = "045739d074b8722891c307e8e75c9607e0b55a80778\
//     //             b42ef5f4640d4949dbf3992f6083b729baef9e9545c4\
//     //             e95590616fd382662a09653f2a966ff524989ae8c0f"
//     //     .as_bytes()
//     //     .to_vec();

//     Ok(ret)
// }

pub fn update(ctx: ContractCtx, request: CtrRequest) -> Result<Vec<u8>, ContractError> {
    return Ok(vec![1]);
    // let mut storage = vec![];
    // match request.req_type.as_ref() {
    //     "add_validator" => {
    //         return handle_add_validator(&mut storage, request.args);
    //     }
    //     _ => {
    //         return Err(format!("Wrong request type has been found").into());
    //     }
    // };
}

// fn handle_add_validator(
//     storage: &mut Storage,
//     args: RequestArgs,
// ) -> Result<Vec<u8>, ContractError> {
//     let mut validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

//     let add_validator_params: AddValidatorParams = serde_json::from_slice(&args)?;

//     validator_storage
//         .validators
//         .push(add_validator_params.validator);

//     *storage = serde_json::to_vec(&validator_storage)?;

//     Ok(vec![])
// }
