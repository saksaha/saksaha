use std::alloc::Layout;

// use sak_contract::Storage;
use serde::{Deserialize, Serialize};

// pub struct Storage {
//     state: String,
// }

// impl Storage {
//     pub fn set_state(&self, str: String) {
//         println!("str: {}", str);
//     }
// }

#[derive(Serialize, Deserialize)]
pub struct State {
    pub validators: Vec<String>,
}

pub fn init(ptr: i32, len: i32) {
    let state = State {
        validators: vec!["person_1".into(), "person_2".into()],
    };

    // storage.set_state(serde_json::to_string(&state).unwrap());
    let a = [0, 0];
    // a.as_ptr()
}
