use crate::{CoinCircuit, CoinProof, CM_TREE_DEPTH};
use crate::{MerkleTree, ProofError};
use rand::rngs::OsRng;
use rand::RngCore;
use sak_crypto::{
    groth16, AllocatedBit, Circuit, ConstraintSystem, Proof, SynthesisError,
};
use sak_crypto::{mimc, Parameters};
use sak_crypto::{Bls12, Hasher, Scalar};
use std::fs::File;
use std::io::Write;

#[test]
pub fn mimc_test() {
    let proof0 = CoinProof::generate_proof(0);

    assert!(CoinProof::verify_proof(&proof0));
}
