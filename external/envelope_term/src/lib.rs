mod app;
mod db;
mod inputs;
mod io;
pub mod pconfig;
pub mod term;
mod views;

#[cfg(test)]
mod tests;

pub type EnvelopeError = Box<dyn std::error::Error + Send + Sync>;

pub const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
