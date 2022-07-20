mod block;
mod block_candidate;
mod hashable;
pub mod tx;
mod u8_array;

pub use block::*;
pub use block_candidate::*;
pub use hashable::*;
pub use tx::*;
pub use u8_array::*;

pub type TypesError = Box<dyn std::error::Error + Send + Sync>;

// pub type MimcDigest = [u8; 32];
