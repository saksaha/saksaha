mod vm;

pub use vm::*;

pub(crate) type BoxedError = Box<dyn std::error::Error + Send + Sync>;
