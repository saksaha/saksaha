use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ContractError {
    pub err_msg: String,
}

impl ContractError {
    pub fn new(err_msg_input: String) -> Self {
        ContractError {
            err_msg: format!("$$__{}", err_msg_input),
        }
    }
}
