mod coin;
mod testing;

pub use coin::*;

pub type ProofTypeError = Box<dyn std::error::Error + Send + Sync>;
