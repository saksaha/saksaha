mod circuit;
mod circuit_1_to_2;
mod proof;

pub use circuit::*;
pub use circuit_1_to_2::*;
pub use proof::*;

pub const CM_TREE_DEPTH: u32 = 5;
pub const CM_TREE_CAPACITY: usize = 2_usize.pow(CM_TREE_DEPTH as u32);
