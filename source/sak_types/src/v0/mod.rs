mod balance;
mod block;
mod block_candidate;
mod coin_record;
pub mod tx;

pub use balance::*;
pub use block::*;
pub use block_candidate::*;
pub use coin_record::*;
pub use tx::*;

pub type TypesError = Box<dyn std::error::Error + Send + Sync>;
