mod coin_proof;
mod merkle;

#[cfg(test)]
mod tests;

pub use coin_proof::*;
pub use merkle::*;

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
