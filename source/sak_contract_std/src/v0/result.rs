use serde::{Deserialize, Serialize};

pub type ContractError = Box<dyn std::error::Error + Send + Sync>;

pub const ERROR_PLACEHOLDER: [u8; 6] = [1, 2, 3, 4, 5, 6];

// #[derive(Serialize, Deserialize, Debug, PartialEq)]
// pub enum Status {
//     SUCCESS,
//     ERROR,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ContractResult {
//     pub data: Vec<u8>,
//     // pub status: Status,
// }

// impl ContractResult {
//     pub fn resolve(self) -> Result<Vec<u8>, ContractError> {
//         if self.status == Status::SUCCESS {
//             return Ok(self.data);
//         } else {
//             let err: InvokeError = serde_json::from_slice(&self.data)?;

//             return Err(err.err_msg.into());
//         }
//     }
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct InvokeError {
//     pub err: String,
// }

// impl From<serde_json::Error> for InvokeError {
//     fn from(src: serde_json::Error) -> InvokeError {
//         InvokeError {
//             err: src.to_string(),
//         }
//     }
// }

// impl From<String> for InvokeError {
//     fn from(src: String) -> InvokeError {
//         InvokeError { err: src }
//     }
// }

// impl InvokeError {
//     pub fn new(err_msg_input: String) -> Self {
//         InvokeError {
//             err_msg: format!("$$__{}", err_msg_input),
//         }
//     }
// }

pub fn make_error_vec(err: ContractError) -> Vec<u8> {
    let v = err.to_string().as_bytes().to_vec();

    [ERROR_PLACEHOLDER.to_vec(), v].concat()
}
