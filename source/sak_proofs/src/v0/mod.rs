mod hasher;
mod merkle;
pub mod mimc;
mod proofs;

#[cfg(test)]
mod tests;

pub use hasher::*;
pub use merkle::*;
pub use proofs::*;

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
pub use bls12_381::Scalar;
