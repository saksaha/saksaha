// use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Storage {
    state: String,
}

impl Storage {
    pub fn set_state(&self, str: String) {
        println!("str: {}", str);
    }
}
