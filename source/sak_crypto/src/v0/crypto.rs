use k256::SecretKey;
use k256::{
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::ecdh::SharedSecret,
    EncodedPoint, Secp256k1,
};
use sha3::{Digest, Keccak256, Sha3_256};
use std::{fmt::Write, num::ParseIntError};

pub type PublicKey = k256::PublicKey;

pub fn keccak256(data: &[u8]) -> String {
    let mut hasher = Keccak256::default();
    hasher.update(data);

    let result = {
        let h = hasher.finalize();
        format!("{:x}", h)
    };

    result
}

// pub fn generate_key() -> SecretKey {
//     let secret = SecretKey::random(&mut OsRng);
//     return secret;
// }

// pub fn create_addr_key_pair(pk: &String) -> (String, String) {
//     let d = pk[2..].as_bytes();
//     let h = keccak256(d);
//     h[24..].to_string()
// }

// pub fn generate_key_pair() -> (SecretKey, PublicKey) {
//     let secret = SecretKey::random(&mut OsRng);
//     let public_key = secret.public_key();

//     (secret, public_key)
// }

// pub fn encode_into_key_pair(sk: SecretKey) -> (String, String) {
//     let pk = sk.public_key();

//     let sk_str = encode_hex(sk.to_bytes().as_slice());
//     let pk_str = encode_hex(pk.to_encoded_point(false).as_bytes());

//     return (sk_str, pk_str);
// }

pub fn decode_hex(s: &String) -> std::result::Result<Vec<u8>, ParseIntError> {
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

pub fn convert_public_key_str_into_public_key(
    public_key_str: &String,
) -> Result<PublicKey, String> {
    let pk_decoded = match decode_hex(public_key_str) {
        Ok(p) => p,
        Err(err) => {
            return Err(format!(
                "Error decoding public key string, err: {}",
                err
            ))
        }
    };

    match PublicKey::from_sec1_bytes(pk_decoded.as_slice()) {
        Ok(p) => return Ok(p),
        Err(err) => {
            return Err(format!(
                "Could not create public key out of byte array, err: {}",
                err
            ));
        }
    }
}

pub fn convert_public_key_to_verifying_key(
    public_key_bytes: [u8; 65],
) -> Result<VerifyingKey, String> {
    let encoded_point = match EncodedPoint::from_bytes(public_key_bytes) {
        Ok(e) => e,
        Err(err) => {
            return Err(format!(
                "Error making EncodedPoint from bytes, err: {}",
                err
            ));
        }
    };

    let verifying_key = match VerifyingKey::from_encoded_point(&encoded_point) {
        Ok(v) => v,
        Err(err) => {
            return Err(format!(
                "Cannot create VerifyingKey from encoded point, \
                        err: {}",
                err
            ));
        }
    };

    Ok(verifying_key)
}

pub fn make_signature(signing_key: SigningKey, data: &[u8]) -> Signature {
    signing_key.sign(data)
}

pub fn verify(
    verifying_key: VerifyingKey,
    data: &[u8],
    sig: &Signature,
) -> Result<(), String> {
    match verifying_key.verify(data, sig) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

pub fn compute_hash(values: &[impl AsRef<[u8]>]) -> String {
    let mut hasher = Sha3_256::new();

    for v in values {
        hasher.update(v);
    }

    let result = {
        let h = hasher.finalize();
        format!("{:x}", h)
    };

    return result;
}

pub fn make_shared_secret(
    my_secret_key: &SecretKey,
    her_public: PublicKey,
) -> SharedSecret<Secp256k1> {
    k256::elliptic_curve::ecdh::diffie_hellman(
        my_secret_key.to_secret_scalar(),
        her_public.as_affine(),
    )
}
