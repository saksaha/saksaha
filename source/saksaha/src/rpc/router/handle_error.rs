use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub(crate) struct HandleError {
    // pub code: &'a str,
    pub msg: String,
}

