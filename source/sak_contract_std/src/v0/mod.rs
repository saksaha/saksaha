mod macros;
mod request;

pub use request::*;
pub use std::error::Error;

pub(crate) type BoxedError = Box<dyn std::error::Error + Send + Sync>;
