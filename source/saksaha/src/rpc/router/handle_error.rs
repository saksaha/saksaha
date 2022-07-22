use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct HandleError {
    // pub code: &'a str,
    pub msg: String,
}
