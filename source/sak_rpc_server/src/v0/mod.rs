mod server;

pub use server::*;

pub type RPCServerError = Box<dyn std::error::Error + Send + Sync>;
