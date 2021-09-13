use crate::{
    common::errors::Error,
    err_res,
};
pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{Signature, SigningKey, VerifyingKey},
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};
use rand_core::OsRng;
use std::{fmt::Write, num::ParseIntError};

pub fn make_secret_key_from_bytes(
    bytes: impl AsRef<[u8]>,
) -> Result<SecretKey, Error> {
    match SecretKey::from_bytes(bytes) {
        Ok(s) => return Ok(s),
        Err(err) => {
            return err_res!("Error making secret out of bytes, err: {}", err);
        }
    }
}

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
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

pub fn encode_key_pair(sk: SecretKey) -> (String, String) {
    let pk = sk.public_key();

    let sk_str = encode_hex(sk.to_bytes().as_slice());
    let pk_str = encode_hex(pk.to_encoded_point(false).as_bytes());

    // print!("11, {}\n{}\n", sk_str, pk_str);

    return (sk_str, pk_str);
}

pub fn generate_key() -> SecretKey {
    let secret = SecretKey::random(&mut OsRng);
    return secret;
}

pub fn to_hex(_: EphemeralSecret) {
    // let pk = secret.public_key();
    // secret.
    // EncodedPoint::from(secret);
    // let pk = EncodedPoint::from(secret.public_key());
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
            // Signing
            let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`
            let message = b"ECDSA proves knowledge of a secret number in the context of a single message";

            // Note: the signature type must be annotated or otherwise inferrable as
            // `Signer` has many impls of the `Signer` trait (for both regular and
            // recoverable signature types).
            let signature: Signature = signing_key.sign(message);

            // Verification
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
