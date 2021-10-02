pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{Signature, SigningKey, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};
use rand_core::OsRng;
use std::{fmt::Write, num::ParseIntError};

use crate::{common::Result, err_res};

pub struct Crypto;

impl Crypto {
    pub fn generate_key() -> SecretKey {
        let secret = SecretKey::random(&mut OsRng);
        return secret;
    }

    pub fn encode_into_key_pair(sk: SecretKey) -> (String, String) {
        let pk = sk.public_key();

        let sk_str = Crypto::encode_hex(sk.to_bytes().as_slice());
        let pk_str = Crypto::encode_hex(pk.to_encoded_point(false).as_bytes());

        return (sk_str, pk_str);
    }

    pub fn decode_hex(
        s: String,
    ) -> std::result::Result<Vec<u8>, ParseIntError> {
        (0..s.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
            .collect()
    }

    pub fn encode_hex(bytes: &[u8]) -> String {
        let mut s = String::with_capacity(bytes.len() * 2);
        for &b in bytes {
            write!(&mut s, "{:02x}", b).unwrap();
        }
        s
    }

    pub fn convert_public_key_to_verifying_key(
        public_key_bytes: [u8; 65],
    ) -> Result<VerifyingKey> {
        let encoded_point = match EncodedPoint::from_bytes(public_key_bytes) {
            Ok(e) => e,
            Err(err) => {
                return err_res!(
                    "Error making EncodedPoint from bytes, err: {}",
                    err
                );
            }
        };

        let verifying_key =
            match VerifyingKey::from_encoded_point(&encoded_point) {
                Ok(v) => v,
                Err(err) => {
                    return err_res!(
                    "Cannot create VerifyingKey from encoded point, err: {}",
                    err
                );
                }
            };

        Ok(verifying_key)
    }
}

#[cfg(test)]
mod test {
    use super::{
        EncodedPoint, EphemeralSecret, OsRng, PublicKey, SecretKey, Signature,
        SigningKey, ToEncodedPoint, VerifyingKey,
    };
    use crate::common::testenv;
    use k256::ecdsa::signature::{Signer, Verifier};

    #[test]
    fn it_creates_signature() {
        testenv::run_test(|_| {
            let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
            let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

            let signature: Signature = signing_key.sign(message);
            let verify_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`

            assert!(verify_key.verify(message, &signature).is_ok());
        })
    }

    #[test]
    fn it_creates_shared_secret() {
        testenv::run_test(|_| {
            // Alice
            let alice_secret = EphemeralSecret::random(&mut OsRng);
            let alice_pk_bytes = EncodedPoint::from(alice_secret.public_key());

            let sk = SecretKey::random(&mut OsRng);
            let sk_bytes = sk.to_bytes();

            print!("secret key: {:?}\n", sk);
            print!("secret key bytes: {:?}\n", alice_pk_bytes);

            let sk_rec = SecretKey::from_bytes(sk_bytes).unwrap();
            print!("recovered secret key {:?}\n", sk_rec);

            let public_key = sk.public_key();
            let enc_point = public_key.to_encoded_point(false);
            let enc_point_bytes = enc_point.as_bytes();

            print!("public key: {:?}\n", public_key);
            print!("encoded point: {:?}\n", enc_point);
            print!("encoded point as bytes: {:?}\n", enc_point_bytes);

            let public_key_rec =
                PublicKey::from_sec1_bytes(enc_point_bytes).unwrap();
            let enc_point_rec = public_key_rec.to_encoded_point(false);

            print!("public key rec: {:?}\n", public_key_rec);
            print!("encoded point rec: {:?}\n", &enc_point_rec);

            assert_eq!(enc_point, enc_point_rec);
        });
    }
}
