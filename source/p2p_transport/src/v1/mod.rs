mod net;
pub mod transport;

pub use net::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub(crate) type Result<T> = std::result::Result<T, Error>;
