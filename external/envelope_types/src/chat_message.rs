use crate::{Date, PublicKey};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub date: Date,
    pub user: PublicKey,
    pub msg: String,
}
