pub mod testing;
mod tx;
mod tx_candidate;
mod tx_type;
mod utils;

pub use tx::*;
pub use tx_candidate::*;
pub use tx_type::*;

pub type TxHash = String;

pub type BlockHash = String;

pub type BlockHeight = u128;

pub type CtrAddr = String;
