use sak_contract_std::{contract_bootstrap, Request};
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

    let new_ptr = new_storage_bytes_vec.as_mut_ptr();
    let len = new_storage_bytes_vec.clone().len();

    std::mem::forget(new_storage_bytes_vec);

    (new_ptr, len as i32)
}

// takes storage and request object
#[no_mangle]
pub unsafe extern "C" fn query(
    // storage
    ptr_storage: *mut u8,
    len_storage: usize,
    // request
    ptr_request: *mut u8,
    len_request: usize,
) -> (*mut u8, i32) {
    // let mut msg = String::from("aaaa");
    // let ptr = msg.as_mut_ptr();
    // let len = msg.len();
    // std::mem::forget(msg);
    // (ptr, len as i32)

    // =-=-= Storage =-=-=
    let storage_bytes_vec = Vec::from_raw_parts(
        ptr_storage, //
        len_storage,
        len_storage,
    );

    let storage_serialized = match String::from_utf8(storage_bytes_vec) {
        Ok(s) => s,
        Err(err) => {
            panic!("Cannot serialize storage, err: {}", err);
        }
    };

    let storage_hashmap: HashMap<&str, String> =
        match serde_json::from_str(&storage_serialized.as_str()) {
            Ok(s) => s,
            Err(err) => {
                panic!("Cannot Deserialize HashMap from storage, err: {}", err);
            }
        };

    // =-=-= Request =-=-=
    let request_bytes_vec = Vec::from_raw_parts(
        ptr_request, //
        len_request,
        len_request,
    );

    let request_serialized = match String::from_utf8(request_bytes_vec) {
        Ok(s) => s,
        Err(err) => {
            panic!("Cannot serialize storage, err: {}", err);
        }
    };
    // =-=-=-=-=-=-=-=-=-=

    let mut msg = String::from("aaaa");
    let ptr = msg.as_mut_ptr();
    let len = msg.len();
    std::mem::forget(msg);
    (ptr, len as i32)

    /*
    let request_struct: Request =
        match serde_json::from_str(&request_serialized.as_str()) {
            Ok(s) => s,
            Err(err) => {
                panic!("Cannot Deserialize HashMap from storage, err: {}", err);
            }
        };

    // =-=-= Validators =-=-=
    let validators_string = match storage_hashmap.get("validators") {
        Some(v) => v,
        None => {
            panic!("validators should be initialized");
        }
    };

    // let validators_string = serde_json::to_value(vec!["deadbeef".to_string()])
    //     .unwrap()
    //     .to_string();

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

    let mut validator = match request_struct.ty {
        "get_validator" => {
            // return validator public key
            validators[0].clone()
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };

    let ptr = validator.as_mut_ptr();

    let len = validator.len();

    std::mem::forget(validator);

    (ptr, len as i32)
    */
}
