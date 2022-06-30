// #[derive(Debug)]
// pub struct StateUpdate {
//     pub ctr_addr: String,
//     pub new_state: String,
// }

use std::collections::HashMap;

pub type StateUpdate = HashMap<String, String>;
