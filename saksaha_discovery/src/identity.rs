use crypto::SecretKey;
pub use k256::ecdsa::Signature;

pub trait Identity {
    fn secret_key(&self) -> &SecretKey;

    fn public_key_bytes(&self) -> [u8; 65];

    fn sig(&self) -> Signature;
}
