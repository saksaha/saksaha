use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractError {
    pub err_msg: String,
}

impl From<serde_json::Error> for ContractError {
    fn from(src: serde_json::Error) -> ContractError {
        ContractError {
            err_msg: src.to_string(),
        }
    }
}

impl ContractError {
    pub fn new(err_msg_input: String) -> Self {
        ContractError {
            err_msg: format!("$$__{}", err_msg_input),
        }
    }
}
