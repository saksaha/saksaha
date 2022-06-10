use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    // state: Vec<String>,
    f1: usize,
}

impl Storage {
    pub fn init() -> Self {
        // Storage { state: vec![] }
        Storage { f1: 0 }
    }
    pub fn set_state(&mut self, msg: String) {
        // self.state.push(msg);
    }
    // pub fn get_state(&self) -> Vec<String> {
    //     self.state.clone()
    // }
    pub fn get_state(&self) -> usize {
        self.f1
    }
}
