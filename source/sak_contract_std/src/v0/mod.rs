mod macros;
mod request;
mod result;
mod storage;

pub use request::*;
pub use result::*;
pub use std::error::Error;
pub use storage::*;

pub type StorageError = Box<dyn std::error::Error + Send + Sync>;
