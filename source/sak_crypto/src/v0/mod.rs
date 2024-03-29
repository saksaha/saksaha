mod crypto;
mod ecies;
pub mod hasher;
mod key;
mod merkle;
mod random;
mod scalar_ext;

#[cfg(test)]
mod tests;

pub use crypto::*;
pub use ecies::*;
pub use key::*;
pub use merkle::*;
pub use rand_core::OsRng;
pub use random::*;
pub use scalar_ext::*;

pub use bls12_381::{Bls12, Scalar};
pub use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    ScalarBytes, SecretKey,
};
use k256::{elliptic_curve::ecdh::SharedSecret as SSecret, Secp256k1};
pub use sha3;

pub use bellman::gadgets::boolean::AllocatedBit;
pub use bellman::groth16::{
    Parameters, PreparedVerifyingKey, Proof, VerifyingKey as Groth16VerifyingKey,
};
pub use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
pub use pairing::MultiMillerLoop;

// type aliases
pub type PublicKey = k256::PublicKey;
pub type SharedSecret = SSecret<Secp256k1>;
pub type CryptoError = Box<dyn std::error::Error + Send + Sync>;
