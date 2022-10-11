mod invoke;

pub use invoke::*;

pub(crate) type VMInterfaceError = Box<dyn std::error::Error + Send + Sync>;
