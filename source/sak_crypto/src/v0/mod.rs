mod utils;

#[cfg(test)]
mod tests;

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
pub use utils::*;

pub type PublicKey = k256::PublicKey;
pub type SharedSecret = SSecret<Secp256k1>;
