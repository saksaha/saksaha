use serde::Serialize;

#[derive(Serialize)]
pub struct Storage {
    // state: String,
}

impl Storage {
    pub fn init() -> Self {
        Storage {}
    }
    pub fn get_state(&self) {}
    pub fn set_state(&self, str: String) {
        println!("str: {}", str);
    }
}
