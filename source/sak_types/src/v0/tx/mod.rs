mod testing;
mod tx;
mod tx_candidate;
mod tx_type;
mod utils;

pub use testing::*;
pub use tx::*;
pub use tx_candidate::*;
pub use tx_type::*;
use type_extension::U8Arr32;

// type aliases
pub type TxHash = String;

pub type BlockHash = String;

pub type BlockHeight = u128;

pub type CtrAddr = String;

pub type TxHeight = u128;

pub type SN = U8Arr32;

pub type CM = U8Arr32;

pub type CM_IDX = u128;
