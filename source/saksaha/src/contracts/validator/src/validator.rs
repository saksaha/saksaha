use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct ValidatorContract {
    validator: Vec<String>,
}

impl ValidatorContract {
    pub fn init(validator: Vec<String>) -> ValidatorContract {
        ValidatorContract { validator }
    }
}
