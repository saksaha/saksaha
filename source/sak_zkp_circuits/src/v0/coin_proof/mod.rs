mod circuits;
mod coin;
mod proof;

pub use circuits::*;
pub use coin::*;

pub const CM_TREE_DEPTH: u32 = 4;

pub const CM_TREE_CAPACITY: usize = 2_usize.pow(CM_TREE_DEPTH as u32);

pub(crate) type CircuitError = Box<dyn std::error::Error + Send + Sync>;
