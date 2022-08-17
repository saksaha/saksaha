mod conn;
pub mod handshake;
mod msg;
mod transport;
pub(crate) mod utils;

#[cfg(test)]
mod tests;

pub use conn::*;
pub use msg::*;
pub use transport::Transport;

pub(crate) type TrptError = Box<dyn std::error::Error + Send + Sync>;
