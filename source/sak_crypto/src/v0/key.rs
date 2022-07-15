use crate::PublicKey;
use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};
use rand_core::OsRng;

pub struct SakKey;

impl SakKey {
    pub fn generate() -> (SecretKey, PublicKey) {
        let secret = SecretKey::random(&mut OsRng);
        let public_key = secret.public_key();

        (secret, public_key)
    }

    pub fn create_acc_addr(pk: &PublicKey) -> String {
        let pk_bytes = pk.to_encoded_point(false).to_bytes();

        let d = &pk_bytes[2..];
        let h = crate::keccak256(d);
        h[24..].to_string()
    }
}
