mod app;
mod credential;
mod db;
mod rpc;
mod wallet;

pub use app::*;
pub use credential::*;

pub type WalletError = Box<dyn std::error::Error + Send + Sync>;
