mod accessor;

pub use accessor::*;

pub(crate) type StoreInterfaceError = Box<dyn std::error::Error + Send + Sync>;
