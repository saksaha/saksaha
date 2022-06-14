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
    let data = Vec::from_raw_parts(ptr, len, len);
    let data_string = String::from_utf8(data).unwrap();
    let mut data_json: HashMap<&str, String> =
        serde_json::from_str(data_string.as_str()).unwrap();

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

    data_json.insert("validators", validators);

    // serialize the data to return a new pointer
    let storage_string = serde_json::to_value(data_json).unwrap().to_string();
    let mut storage_bytes_vec = storage_string.as_bytes().to_owned();

    let ptr_new = storage_bytes_vec.as_mut_ptr();

    let len = storage_bytes_vec.clone().len();

    std::mem::forget(storage_bytes_vec);

    (ptr_new, len as i32)
}

// takes storage and request object
#[no_mangle]
pub unsafe extern "C" fn query(// storage,
    // req
) {
    // match req.ty {
    //     "get_validator" => {
    //         // return validator public key
    //     }
    //     _ => {}
    // }
}
