use crate::{CryptoError, PublicKey, SecretKey};
use aes_gcm_siv::aead::{Aead, NewAead};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
use hkdf::Hkdf;
use k256::elliptic_curve::sec1::ToEncodedPoint;
use rand::{thread_rng, Rng};
use sha2::Sha256;
use sha3::Sha3_256;

/// AES IV/nonce length
pub const AES_IV_LENGTH: usize = 12;
/// AES tag length
pub const AES_TAG_LENGTH: usize = 16;
/// AES IV + tag length
// pub const AES_IV_PLUS_TAG_LENGTH: usize = AES_IV_LENGTH + AES_TAG_LENGTH;
// /// Empty bytes array
// pub const EMPTY_BYTES: [u8; 0] = [];

pub fn derive_aes_key(my_secret: SecretKey, her_public: PublicKey) -> [u8; 32] {
    let shared_secret = k256::elliptic_curve::ecdh::diffie_hellman(
        my_secret.to_secret_scalar(),
        her_public.as_affine(),
    );

    let mut material = Vec::new();

    material.extend_from_slice(shared_secret.as_bytes());

    let aes_key = {
        let h = Hkdf::<Sha256>::new(None, material.as_slice());
        let mut out = [0u8; 32];
        h.expand(&[], &mut out).unwrap();
        out
    };

    aes_key
}

pub fn aes_encrypt(
    aes_key: &[u8; 32],
    plaintext: &[u8],
) -> Result<Vec<u8>, CryptoError> {
    let iv: [u8; AES_IV_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let key = Key::from_slice(aes_key);
    let cipher = Aes256GcmSiv::new(key);

    let nonce = Nonce::from_slice(&iv); // 96-bits; unique per message

    let ciphertext = match cipher.encrypt(nonce, plaintext.as_ref()) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!("cannot encrypt plaintext : {}", err).into());
        }
    };

    Ok(ciphertext)
}

pub fn aes_decrypt(
    aes_key: &[u8; 32],
    ciphertext: &[u8],
) -> Result<String, CryptoError> {
    let key = Key::from_slice(aes_key);
    let cipher = Aes256GcmSiv::new(key);

    let iv: [u8; AES_IV_LENGTH] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let nonce = Nonce::from_slice(&iv); // 96-bits; unique per message

    let plaintext: Vec<u8> = match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(c) => c,
        Err(err) => {
            return Err(format!("cannot decrypt ciphertext : {}", err).into());
        }
    };

    let a = String::from_utf8(plaintext).unwrap();
    Ok(a)
}