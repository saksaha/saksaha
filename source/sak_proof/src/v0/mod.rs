mod coin_proof;
mod testing;

#[cfg(test)]
mod tests;

pub use coin_proof::*;
pub use sak_proof_circuit::{NewCoin, OldCoin};

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
