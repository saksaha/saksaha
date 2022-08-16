mod app;
mod config;
mod credential;
mod db;
mod pconfig;
mod rpc;
mod tests;
mod wallet;

pub use app::*;
pub use config::*;
pub use credential::*;
pub use pconfig::*;

pub type WalletError = Box<dyn std::error::Error + Send + Sync>;
