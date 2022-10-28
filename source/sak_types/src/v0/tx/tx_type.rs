use serde::{Deserialize, Serialize};

#[repr(u8)]
#[derive(Debug, Serialize, Deserialize)]
pub enum TxType {
    Invalid = 0,
    Mint,
    Pour,
}

impl From<u8> for TxType {
    fn from(b: u8) -> TxType {
        match b {
            1 => Self::Mint,
            2 => Self::Pour,
            _ => Self::Invalid,
        }
    }
}
