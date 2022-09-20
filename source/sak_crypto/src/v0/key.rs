use crate::OsRng;
use crate::PublicKey;
use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};

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

    pub fn create_acc_addr_from_pk_str(pk: &String) -> String {
        let pk_bytes = pk.as_bytes();
        let d = &pk_bytes[2..];
        let h = crate::keccak256(d);
        h[24..].to_string()
    }

    pub fn foo() -> String {
        String::from("power 1")
    }
}
