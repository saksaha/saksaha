use serde::{Deserialize, Serialize};
use std::collections::HashMap;

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

pub type Storage = HashMap<String, String>;
