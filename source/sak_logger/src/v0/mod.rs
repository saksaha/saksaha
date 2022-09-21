mod formatters;
mod logger;
mod macros;
mod utils;

pub use logger::*;

pub type LoggerError = Box<dyn std::error::Error + Send + Sync>;

pub const RUST_LOG_ENV: &str = "sak_=debug,saksaha_=debug,hyper_=debug";
