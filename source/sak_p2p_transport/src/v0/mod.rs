pub mod handshake;
mod msg;
pub(crate) mod net;
mod transport;
pub(crate) mod utils;

#[cfg(test)]
mod tests;

pub use msg::*;
pub use net::*;
pub use transport::Transport;

pub(crate) type TrptError = Box<dyn std::error::Error + Send + Sync>;
