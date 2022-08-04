mod middlewares;
mod server;

pub use middlewares::*;
pub use server::*;

pub type RPCServerError = Box<dyn std::error::Error + Send + Sync>;
