mod app;
mod inputs;
mod io;
mod pconfig;
pub mod term;
mod views;

pub type EnvelopeError = Box<dyn std::error::Error + Send + Sync>;
pub const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
