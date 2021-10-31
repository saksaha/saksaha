pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};

pub const PUBLIC_KEY_LEN: usize = 65;

pub trait Identity {
    fn secret_key(&self) -> &SecretKey;

    fn public_key_bytes(&self) -> [u8; 65];

    fn sig(&self) -> Signature;
}
