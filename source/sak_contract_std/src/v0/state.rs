use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
pub struct State {
    pub validators: BTreeMap<Vec<u8>, Vec<u8>>,
}
