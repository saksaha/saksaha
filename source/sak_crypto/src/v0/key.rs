use crate::{encode_hex, PublicKey};
use crate::{CryptoError, OsRng};
use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};
use serde::{Deserialize, Serialize};

pub struct SakKey;

#[derive(Serialize, Deserialize, Debug)]
pub struct Credential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
}

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

impl Credential {
    pub fn new_random() -> Result<Credential, CryptoError> {
        let (sk, pk) = SakKey::generate();
        let secret = encode_hex(&sk.to_bytes());
        let public_key = encode_hex(&pk.to_encoded_point(false).to_bytes());

        let acc_addr = SakKey::create_acc_addr(&pk);

        let c = Credential {
            public_key,
            secret,
            acc_addr,
        };

        Ok(c)
    }
}
