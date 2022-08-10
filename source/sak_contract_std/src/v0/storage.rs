use crate::StorageError;
use std::collections::HashMap;

// pub type Storage = HashMap<String, String>;
pub type Storage = Vec<u8>;

// pub fn parse(data: &[u8]) -> Result<Storage, StorageError> {
//     match serde_json::from_slice(data) {
//         Ok(s) => Ok(s),
//         Err(err) => {
//             return Err(err.into());
//         }
//     }
// }
