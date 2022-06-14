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
pub unsafe extern "C" fn query(//
    // storage,
    // req
) {
    // match req.ty {
    //     "get_validator" => {
    //         // return validator public key
    //     }
    //     _ => {}
    // }
}
