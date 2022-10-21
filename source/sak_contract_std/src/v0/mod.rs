mod ctr_fn;
mod data;
mod macros;
mod request;
mod result;
mod size;
mod storage;
pub mod symbols;

pub use ctr_fn::*;
pub use data::*;
pub use request::*;
pub use result::*;
pub use size::*;
pub use std::error::Error;
pub use storage::*;

pub type StorageError = Box<dyn std::error::Error + Send + Sync>;

pub type ContractError = Box<dyn std::error::Error + Send + Sync>;
