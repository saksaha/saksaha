mod accessor;

pub use accessor::*;

pub(crate) type StoreError = Box<dyn std::error::Error + Send + Sync>;
