mod balance;
mod block;
mod block_candidate;
mod coin_record;
mod contract;
mod event;
mod testing;
mod tx;

pub use balance::*;
pub use block::*;
pub use block_candidate::*;
pub use coin_record::*;
pub use contract::*;
pub use event::*;
pub use testing::*;
pub use tx::*;

pub type TypesError = Box<dyn std::error::Error + Send + Sync>;
