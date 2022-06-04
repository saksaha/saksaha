use sak_contract::Storage;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub validators: Vec<String>,
}

#[wasm_bindgen]
pub fn init(storage: Storage) {
    let state = State {
        validators: vec!["person_1".into(), "person_2".into()],
    };

    storage.set_state(serde_json::to_string(&state).unwrap());
}

pub fn query() {}
