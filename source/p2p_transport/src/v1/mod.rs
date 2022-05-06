mod net;
pub mod ops;
pub mod transport;

pub use net::*;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

pub(crate) type Result<T> = std::result::Result<T, Error>;
