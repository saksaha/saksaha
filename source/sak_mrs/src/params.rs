use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ReserveSlotParams {
    pub public_key: String,
}
