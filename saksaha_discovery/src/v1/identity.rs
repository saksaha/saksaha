use saksaha_crypto::SecretKey;
use k256::ecdsa::Signature;

pub const PUBLIC_KEY_LEN: usize = 65;

pub trait Identity {
    fn secret_key(&self) -> &SecretKey;

    fn public_key_bytes(&self) -> [u8; 65];

    fn sig(&self) -> Signature;
}
