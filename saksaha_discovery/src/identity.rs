use crypto::SecretKey;

pub trait Identity {
    fn secret_key(&self) -> &SecretKey;
    fn public_key_bytes(&self) -> [u8; 65];
}
