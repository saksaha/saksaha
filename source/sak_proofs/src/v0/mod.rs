mod merkle;
mod proofs;

#[cfg(test)]
mod tests;

pub use merkle::*;
pub use proofs::*;

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
