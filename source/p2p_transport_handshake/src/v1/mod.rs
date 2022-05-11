pub mod ops;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
