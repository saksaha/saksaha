use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Storage {
    state: String,
}

impl Storage {
    pub fn init() -> Self {
        Storage {
            state: "initiated storage".to_string(),
        }
    }
    pub fn set_state(&mut self, msg: String) {
        self.state = msg;
    }
    pub fn get_state(&self) -> String {
        self.state.clone()
    }
}
