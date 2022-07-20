use crate::MerkleTree;
use crate::CM_TREE_DEPTH;
use pairing::MultiMillerLoop;
use rand::rngs::OsRng;
use sak_crypto::{
    groth16, mimc, AllocatedBit, Bls12, Circuit, ConstraintSystem,
    Groth16VerifyingKey, Parameters, PreparedVerifyingKey, Proof, Scalar,
    SynthesisError,
};
use std::convert::TryInto;
use std::fs::File;
use std::io::Write;

pub struct CoinProof;

impl CoinProof {
    pub fn make_verifying_key<E: MultiMillerLoop>(
        vk: &Groth16VerifyingKey<E>,
    ) -> PreparedVerifyingKey<E> {
        groth16::prepare_verifying_key(vk)
    }
}
