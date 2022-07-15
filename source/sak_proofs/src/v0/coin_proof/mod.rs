mod circuit;
mod proof;

pub use circuit::*;
pub use proof::*;

pub const CM_TREE_DEPTH: usize = 5;
pub const CM_TREE_CAPACITY: usize = 2_usize.pow(CM_TREE_DEPTH as u32);
