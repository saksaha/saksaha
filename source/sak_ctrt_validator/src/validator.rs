use sak_contract_std::{contract_bootstrap, Request, Storage};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

contract_bootstrap!();

// validator init: takes storage
#[no_mangle]
pub unsafe extern "C" fn init(
    // storage
    ptr: *mut u8,
    len: usize,
) -> (*mut u8, i32) {
    // get data from the pointer
    let storage = Vec::from_raw_parts(ptr, len, len);
    let storage_serialized = String::from_utf8(storage).unwrap();
    let mut storage_hashmap: HashMap<&str, String> =
        serde_json::from_str(&storage_serialized.as_str()).unwrap();

    let validators = match serde_json::to_string(&vec![String::from(
        "\
            046885b904a8b8cdd17cc40078ed11421\
            4586f197a664d6aa33d4b46cc3b712afc\
            def3d4d808bc7843beaea9e1a4c5ddeea\
            47cbd27ea1af5ca13719a2f42c39167\
            ",
    )]) {
        Ok(s) => s,
        Err(err) => panic!("Cannot serialize validators, err: {}", err),
    };

    storage_hashmap.insert("validators", validators);

    // serialize the data to return a new pointer
    let new_storage_serialized =
        serde_json::to_value(storage_hashmap).unwrap().to_string();
    let mut new_storage_bytes_vec =
        new_storage_serialized.as_bytes().to_owned();

    let storage_ptr = new_storage_bytes_vec.as_mut_ptr();
    let storage_len = new_storage_bytes_vec.len();

    std::mem::forget(new_storage_bytes_vec);

    (storage_ptr, storage_len as i32)
}

// takes storage and request object
#[no_mangle]
pub unsafe extern "C" fn query(
    // storage
    storage_ptr: *mut u8,
    storage_len: usize,
    // request
    request_ptr: *mut u8,
    request_len: usize,
) -> (*mut u8, i32) {
    println!("{:?}, {}", storage_ptr, storage_len);

    // let mut msg = String::from("aaaa");
    // let ptr = msg.as_mut_ptr();
    // let len = msg.len();
    // std::mem::forget(msg);
    // (ptr, len as i32)

    // =-=-= Storage =-=-=
    let storage_bytes_vec = Vec::from_raw_parts(
        storage_ptr, //
        storage_len,
        storage_len,
    );

    let storage_serialized = match String::from_utf8(storage_bytes_vec) {
        Ok(s) => s,
        Err(err) => {
            panic!("Cannot serialize storage, err: {}", err);
        }
    };

    let storage: Storage =
        match serde_json::from_str(&storage_serialized.as_str()) {
            Ok(s) => s,
            Err(err) => {
                panic!(
                    "Cannot Deserialize `HashMap` from storage, err: {}",
                    err
                );
            }
        };

    // =-=-= Request =-=-=
    let request_bytes_vec = Vec::from_raw_parts(
        request_ptr, //
        request_len,
        request_len,
    );

    let request_serialized = match String::from_utf8(request_bytes_vec) {
        Ok(s) => s,
        Err(err) => {
            panic!("Cannot serialize storage, err: {}", err);
        }
    };

    let request: Request =
        match serde_json::from_str(&request_serialized.as_str()) {
            Ok(s) => s,
            Err(err) => {
                panic!(
                    "Cannot Deserialize `Storage` from request, err: {}",
                    err
                );
            }
        };

    // =-=-= Validators =-=-=
    let validators_string = match storage.get("validators") {
        Some(v) => v,
        None => {
            panic!("validators should be initialized");
        }
    };

    let validators: Vec<String> =
        match serde_json::from_str(validators_string.as_str()) {
            Ok(v) => v,
            Err(err) => {
                panic!(
                    "validators should be contained in vector, err: {}",
                    err
                );
            }
        };

    let mut validator = match request.ty {
        "get_validator" => {
            // return validator public key
            validators[0].clone()
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };

    let validator_ptr = validator.as_mut_ptr();
    let validator_len = validator.len();
    std::mem::forget(validator);
    (validator_ptr, validator_len as i32)
}
