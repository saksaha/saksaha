mod mint_tx;
mod pour_tx;
// mod testing;
mod tx;
mod tx_candidate;
mod tx_type;
mod utils;

pub use mint_tx::*;
pub use pour_tx::*;
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

pub type Sn = U8Arr32;

pub type Cm = U8Arr32;

pub type CmIdx = u128;
