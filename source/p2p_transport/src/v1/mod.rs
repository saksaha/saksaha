pub mod connection;
pub(crate) mod frame;
pub mod handshake;
pub(crate) mod msg;
pub mod transport;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

pub(crate) type Result<T> = std::result::Result<T, Error>;
