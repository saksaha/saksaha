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
    validators: Vec<String>,
}

contract_bootstrap!();

// #[no_mangle]
// pub unsafe extern "C" fn init() -> (*mut u8, i32) {
//     let mut storage_init = Storage::with_capacity(10);

//     let validators_init = match serde_json::to_string(&vec![String::from(
//         "\
//             045739d074b8722891c307e8e75c9607e0b55a80778b42ef5f4640d4949dbf399\
//     2f6083b729baef9e9545c4e95590616fd382662a09653f2a966ff524989ae8c0f\
//             ",
//     )]) {
//         Ok(s) => s,
//         Err(err) => panic!("Cannot serialize validators, err: {}", err),
//     };

//     storage_init.insert(String::from("validators"), validators_init);

//     let storage_init_serialized =
//         serde_json::to_value(storage_init).unwrap().to_string();
//     let mut storage_init_bytes_vec =
//         storage_init_serialized.as_bytes().to_owned();

//     let storage_ptr = storage_init_bytes_vec.as_mut_ptr();
//     let storage_len = storage_init_bytes_vec.len();

//     std::mem::forget(storage_init_bytes_vec);

//     (storage_ptr, storage_len as i32)
// }

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
    match request.req_type.as_ref() {
        "get_validator" => {
            return handle_get_validator(storage);
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };
}

fn handle_get_validator(storage: Storage) -> Result<String, ContractError> {
    let validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;

    let validator = validator_storage.validators[0];

    Ok(validator)
}

define_execute!();
pub fn execute2(
    storage: &mut Storage,
    request: Request,
) -> Result<(), ContractError> {
    match request.req_type.as_ref() {
        "add_validator" => {
            return handle_add_validator(&mut storage, request.args);
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };
}

// #[no_mangle]
// pub unsafe extern "C" fn execute(
//     storage_ptr: *mut u8,
//     storage_len: usize,
//     request_ptr: *mut u8,
//     request_len: usize,
// ) -> (*mut u8, i32) {
//     let storage_bytes_vec = Vec::from_raw_parts(
//         storage_ptr, //
//         storage_len,
//         storage_len,
//     );

//     let storage_serialized = match String::from_utf8(storage_bytes_vec) {
//         Ok(s) => s,
//         Err(err) => {
//             panic!("Cannot serialize storage, err: {}", err);
//         }
//     };

//     let storage: Storage =
//         match serde_json::from_str(&storage_serialized.as_str()) {
//             Ok(s) => s,
//             Err(err) => {
//                 panic!(
//                     "Cannot Deserialize `HashMap` from storage, err: {}",
//                     err
//                 );
//             }
//         };

//     let request_bytes_vec = Vec::from_raw_parts(
//         request_ptr, //
//         request_len,
//         request_len,
//     );

//     let request_serialized = match String::from_utf8(request_bytes_vec) {
//         Ok(s) => s,
//         Err(err) => {
//             panic!("Cannot serialize storage, err: {}", err);
//         }
//     };

//     let request: Request =
//         match serde_json::from_str(&request_serialized.as_str()) {
//             Ok(s) => s,
//             Err(err) => {
//                 panic!(
//                     "Cannot Deserialize `Storage` from request, err: {}",
//                     err
//                 );
//             }
//         };

//     match request.req_type.as_ref() {
//         "add_validator" => {
//             return handle_add_validator(storage, request.args);
//         }
//         // "remove_validator" => {
//         //     return handle_remove_validator(storage);
//         // }
//         _ => {
//             panic!("Wrong request type has been found");
//         }
//     };
// }

fn handle_add_validator(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let validator_storage: ValidatorStorage = serde_json::from_slice(&storage)?;
    // let validators_string = match storage.get_mut("validators") {
    //     Some(v) => v,
    //     None => {
    //         panic!("validators should be initialized");
    //     }
    // };

    // let mut validators_vec: Vec<String> =
    //     match serde_json::from_str(validators_string.as_str()) {
    //         Ok(v) => v,
    //         Err(err) => {
    //             panic!(
    //                 "validators should be contained in vector, err: {}",
    //                 err
    //             );
    //         }
    //     };

    let validator_new = match args.get("validator") {
        Some(v) => {
            if v.len() == VALIDATOR_LEN {
                v
            } else {
                panic!("invalid size of validator")
            }
        }
        None => {
            panic!("args should contain a new validator");
        }
    };

    validators_vec.push(validator_new.clone());

    let validators_result_string =
        serde_json::to_string(&validators_vec).unwrap();

    let mut storage_result = Storage::with_capacity(10);
    storage_result.insert(String::from("validators"), validators_result_string);

    let storage_result_serialized =
        serde_json::to_string(&storage_result).unwrap();

    let mut storage_init_bytes_vec =
        storage_result_serialized.as_bytes().to_owned();

    let storage_ptr = storage_init_bytes_vec.as_mut_ptr();
    let storage_len = storage_init_bytes_vec.len();

    std::mem::forget(storage_init_bytes_vec);

    (storage_ptr, storage_len as i32)
}
