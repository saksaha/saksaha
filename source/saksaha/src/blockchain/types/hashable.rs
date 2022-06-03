use super::{Block, Transaction};
use serde::{Deserialize, Serialize};
use sha3::Sha3_256;

pub(crate) trait Hashable {
    fn get_hash(&self) -> Result<String, String>;
}

// #[derive(Deserialize, Serialize, Debug, Clone)]
// pub(crate) struct Hash {
//     pub hash: String,
// }
