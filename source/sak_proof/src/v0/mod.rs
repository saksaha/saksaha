mod coin_proof;
mod testing;

#[cfg(test)]
mod tests;

pub use coin_proof::*;

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
