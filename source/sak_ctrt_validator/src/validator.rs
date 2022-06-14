use sak_contract_std::contract_bootstrap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

contract_bootstrap!();

// #[derive(Serialize, Deserialize)]
// pub struct Storage {
//     state: Vec<String>,
// }

// impl Storage {
//     pub fn init() -> Self {
//         Storage { state: vec![] }
//     }
//     pub fn set_state(&mut self, msg: String) {
//         self.state.push(msg);
//     }
//     pub fn get_state(&self) -> Vec<String> {
//         self.state.clone()
//     }
// }

// extern "C" {
//     // fn temp(a: i32) -> i32;

//     fn hello(a: i32, b: i32) -> (i32, i32);
// }

// validator init
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

    let validator_1 = String::from(
        "\
        046885b904a8b8cdd17cc40078ed11421\
        4586f197a664d6aa33d4b46cc3b712afc\
        def3d4d808bc7843beaea9e1a4c5ddeea\
        47cbd27ea1af5ca13719a2f42c39167\
        ",
    );

    // let validator_2 = String::from(
    //     "\
    //     0482982b0fdeb31daf3698cd6c64d7c7b\
    //     e747c97e77f9d9df23a66a7ffcec6b510\
    //     9d7adcb57aa4436cc55cf778dfd3874d8\
    //     0e41125b7161a5b76b7c7a09adb74cc\
    //     ",
    // );
    // let validator_3 = String::from(
    //     "\
    //     0482982b0fdeb31daf3698cd6c64d7c7b\
    //     e747c97e77f9d9df23a66a7ffcec6b510\
    //     9d7adcb57aa4436cc55cf778dfd3874d8\
    //     0e41125b7161a5b76b7c7a09adb74cc\
    //     ",
    // );

    data_json.insert("power", validator_1);

    // serialize the data to return a new pointer
    let storage_string = serde_json::to_value(data_json).unwrap().to_string();
    let mut storage_bytes_vec = storage_string.as_bytes().to_owned();

    let ptr_new = storage_bytes_vec.as_mut_ptr();

    let len = storage_bytes_vec.clone().len();

    std::mem::forget(storage_bytes_vec);

    (ptr_new, len as i32)
}
