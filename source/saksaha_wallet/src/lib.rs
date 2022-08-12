mod app;
mod credential;
mod db;
mod rpc;
mod wallet;

pub use app::*;
pub use wallet::*;

pub type WalletError = Box<dyn std::error::Error + Send + Sync>;
