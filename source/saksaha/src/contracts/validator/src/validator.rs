use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ValidatorContract3 {
    f1: Vec<usize>,
}

impl ValidatorContract3 {
    pub fn init() {
        println!("1");
    }
}
