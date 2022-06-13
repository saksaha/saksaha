mod macros;

pub use std::error::Error;

pub(crate) type BoxedError = Box<dyn std::error::Error + Send + Sync>;
