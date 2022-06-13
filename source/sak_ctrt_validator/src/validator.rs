use std::collections::HashMap;

use sak_contract_std::contract_bootstrap;
use serde::{Deserialize, Serialize};

contract_bootstrap!();

#[derive(Serialize, Deserialize)]
pub struct Storage {
    state: Vec<String>,
}

impl Storage {
    pub fn init() -> Self {
        Storage { state: vec![] }
    }
    pub fn set_state(&mut self, msg: String) {
        self.state.push(msg);
    }
    pub fn get_state(&self) -> Vec<String> {
        self.state.clone()
    }
}

// validator init
#[no_mangle]
pub unsafe extern "C" fn init(
    // storage
    ptr: *mut u8,
    len: usize,
) -> (*mut u8, usize) {
    // get data from the pointer
    let data = Vec::from_raw_parts(ptr, len, len);
    let data_string = String::from_utf8(data).unwrap();
    // deserialize the data
    // let mut data_json: Storage =
    //     serde_json::from_str(data_string.as_str()).unwrap();
    let mut data_json: HashMap<String, String> =
        serde_json::from_str(data_string.as_str()).unwrap();

    let validator_1 = String::from(
        "\
        046885b904a8b8cdd17cc40078ed11421\
        4586f197a664d6aa33d4b46cc3b712afc\
        def3d4d808bc7843beaea9e1a4c5ddeea\
        47cbd27ea1af5ca13719a2f42c39167\
        ",
    );

    let validator_2 = String::from(
        "\
        0482982b0fdeb31daf3698cd6c64d7c7b\
        e747c97e77f9d9df23a66a7ffcec6b510\
        9d7adcb57aa4436cc55cf778dfd3874d8\
        0e41125b7161a5b76b7c7a09adb74cc\
        ",
    );
    let validator_3 = String::from(
        "\
        0482982b0fdeb31daf3698cd6c64d7c7b\
        e747c97e77f9d9df23a66a7ffcec6b510\
        9d7adcb57aa4436cc55cf778dfd3874d8\
        0e41125b7161a5b76b7c7a09adb74cc\
        ",
    );

    // edit state
    // data_json.set_state(validator_1);
    // data_json.set_state(validator_2);
    // data_json.set_state(validator_3);
    data_json.insert("power".to_string(), "foo".to_string());

    // serialize the data to return a new pointer
    let storage_string = serde_json::to_value(data_json).unwrap().to_string();
    let mut storage_bytes_vec = storage_string.as_bytes().to_owned();

    let ptr_new = storage_bytes_vec.as_mut_ptr();

    let len = storage_bytes_vec.clone().len();

    std::mem::forget(storage_bytes_vec);

    (ptr_new, len)
}

/// Given a pointer to the start of a byte array and
/// its length, return the sum of its elements.
#[no_mangle]
pub unsafe extern "C" fn array_sum(ptr: *mut u8, len: usize) -> u8 {
    // create a `Vec<u8>` from the pointer to the
    // linear memory and length
    let data = Vec::from_raw_parts(ptr, len, len);
    // actually compute the sum and return it
    data.iter().sum()
}

/// Given a pointer to the start of a byte array and
/// its length, read a string, create its uppercase
/// representation, then return the pointer in
/// memory to it.
#[no_mangle]
pub unsafe extern "C" fn upper(ptr: *mut u8, len: usize) -> *mut u8 {
    // create a `Vec<u8>` from the pointer and length
    // here we could also use Rust's excellent FFI
    // libraries to read a string, but Hfor simplicity,
    // we are using the same method as for plain byte arrays
    let data = Vec::from_raw_parts(ptr, len, len);
    // read a Rust `String` from the byte array,
    let input_str = String::from_utf8(data).unwrap();
    // transform the string to uppercase, then turn it into owned bytes
    let mut upper = input_str.to_ascii_uppercase().as_bytes().to_owned();
    let ptr = upper.as_mut_ptr();
    // take ownership of the memory block where the result string
    // is written and ensure its destructor is not
    // called whe the object goes out of scope
    // at the end of the function
    std::mem::forget(upper);
    // return the pointer to the uppercase string
    // so the runtime can read data from this offset
    ptr
}
