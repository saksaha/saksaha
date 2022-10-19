mod coin_proof;

pub use coin_proof::*;

pub type CircuitError = Box<dyn std::error::Error + Send + Sync>;
