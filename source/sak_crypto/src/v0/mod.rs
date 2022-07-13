mod crypto;
mod hasher;
pub mod mimc;
mod scalar_ext;

pub use bellman::gadgets::boolean::AllocatedBit;
pub use bellman::groth16::{
    Parameters, PreparedVerifyingKey, Proof, VerifyingKey as Groth16VK,
};
pub use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
pub use bls12_381::{Bls12, Scalar};
pub use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    SecretKey,
};
use k256::{elliptic_curve::ecdh::SharedSecret as SSecret, Secp256k1};
pub use sha3;

pub use crypto::*;
pub use hasher::*;
pub use scalar_ext::*;

pub type PublicKey = k256::PublicKey;
pub type SharedSecret = SSecret<Secp256k1>;

pub type CryptoError = Box<dyn std::error::Error + Send + Sync>;
