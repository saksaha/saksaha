mod app;
mod config;
mod credential;
mod db;
mod envelope;
mod inputs;
mod io;
mod views;
mod wallet_sdk;

#[cfg(test)]
mod tests;

pub use app::*;
pub use config::*;
pub(crate) use envelope::*;

pub type EnvelopeError = Box<dyn std::error::Error + Send + Sync>;

pub const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
