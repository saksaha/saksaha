mod coin_proof;

#[cfg(test)]
mod tests;

pub use bellman::gadgets::boolean::AllocatedBit;
pub use bellman::groth16::{
    Parameters, PreparedVerifyingKey, Proof,
    VerifyingKey as Groth16VerifyingKey,
};
pub use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
pub use bls12_381::{Bls12, Scalar};

// Saksaha zkp circuits
pub use coin_proof::*;
pub use sak_zkp_circuits::{
    get_parent_idx, Hasher, MerkleTree, NewCoin, OldCoin, Path, CM_TREE_DEPTH,
};

pub type ProofError = Box<dyn std::error::Error + Send + Sync>;
