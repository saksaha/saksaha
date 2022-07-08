use crate::{CryptoError, PublicKey, SecretKey};
use aes_gcm::{
    aead::generic_array::GenericArray, AeadInPlace, Aes256Gcm, NewAead,
};
use hkdf::Hkdf;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand::{thread_rng, Rng};
use sha3::Sha3_256;

/// AES IV/nonce length
pub const AES_IV_LENGTH: usize = 16;
/// AES tag length
pub const AES_TAG_LENGTH: usize = 16;
/// AES IV + tag length
pub const AES_IV_PLUS_TAG_LENGTH: usize = AES_IV_LENGTH + AES_TAG_LENGTH;
/// Empty bytes array
pub const EMPTY_BYTES: [u8; 0] = [];

pub fn derive_aes_key(my_secret: SecretKey, her_public: PublicKey) -> [u8; 32] {
    let shared_secret = k256::elliptic_curve::ecdh::diffie_hellman(
        my_secret.to_secret_scalar(),
        her_public.as_affine(),
    );

    // derive key
    let mut material = Vec::new();

    material.extend_from_slice(
        &my_secret.public_key().to_encoded_point(false).to_bytes(),
    );

    material.extend_from_slice(shared_secret.as_bytes());

    let aes_key = {
        let h = Hkdf::<Sha3_256>::new(None, material.as_slice());
        let mut out = [0u8; 32];
        h.expand(&[], &mut out).unwrap();
        out
    };

    aes_key
}

pub fn aes_encrypt(aes_key: &[u8], msg: &[u8]) -> Result<Vec<u8>, CryptoError> {
    println!(
        "aes_encrypt, aes_key ({}): {:?}, msg: {:?}",
        aes_key.len(),
        aes_key,
        msg
    );

    let key = GenericArray::from_slice(aes_key);
    let aead = Aes256Gcm::new(key);

    let mut iv = [0u8; AES_IV_LENGTH];
    thread_rng().fill(&mut iv);

    println!("iv ({}): {:?}", iv.len(), iv);

    let nonce = GenericArray::from_slice(&iv);

    let mut out = Vec::with_capacity(msg.len());
    out.extend(msg);

    match aead.encrypt_in_place_detached(nonce, &EMPTY_BYTES, &mut out) {
        Ok(tag) => {
            let mut output =
                Vec::with_capacity(AES_IV_PLUS_TAG_LENGTH + msg.len());
            output.extend(&iv);
            output.extend(tag);
            output.extend(out);

            return Ok(output);
        }
        Err(err) => {
            return Err(format!("aes ecryption failed, err: {}", err).into());
        }
    };
}
