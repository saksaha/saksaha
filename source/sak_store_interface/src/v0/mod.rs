mod interface;

pub use interface::*;

pub type StoreInterfaceError = Box<dyn std::error::Error + Send + Sync>;
