use crate::{make_public_key_short, CredentialError};
use sak_crypto::{PublicKey, SakKey, SecretKey, Signature, SigningKey, ToEncodedPoint};

// 64 + 1 (flag for whether the key is compressed or not)
pub const PUBLIC_KEY_LEN: usize = 64 + 1;
pub const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct Credential {
    pub secret: String,
    pub public_key_str: String,
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
    pub sig: Signature,
    pub acc_addr: String,
}

impl Credential {
    pub fn new(secret: &String, public_key_str: &String) -> Result<Credential, CredentialError> {
        let secret_bytes = match sak_crypto::decode_hex(&secret) {
            Ok(v) => v,
            Err(err) => {
                return Err(format!("Error making secret key, err: {}", err).into());
            }
        };

        let secret_key = match SecretKey::from_bytes(secret_bytes) {
            Ok(sk) => sk,
            Err(err) => {
                return Err(format!("Error creating SecretKey object, err: {}", err).into());
            }
        };

        let public_key_bytes: [u8; PUBLIC_KEY_LEN] = {
            let b = secret_key.public_key().to_encoded_point(false).to_bytes();

            if b.len() != 65 {
                return Err(
                    format!("Error encoding public key into bytes, size does not fit").into(),
                );
            }

            let mut buf = [0; 65];
            buf.clone_from_slice(&b);
            let pk_encoded = sak_crypto::encode_hex(&b);
            if &pk_encoded != public_key_str {
                return Err(
                    format!("Encoded public key is different from the restored one",).into(),
                );
            }

            buf
        };

        let sig = {
            let signing_key = SigningKey::from(&secret_key);
            let sig = sak_crypto::make_signature(signing_key, SAKSAHA);
            sig
        };

        let public_key = secret_key.public_key();

        let acc_addr = SakKey::create_acc_addr(&public_key);

        let credential = Credential {
            secret: secret.to_owned(),
            secret_key,
            public_key_str: public_key_str.to_owned(),
            public_key,
            public_key_bytes,
            sig,
            acc_addr,
        };

        Ok(credential)
    }

    pub fn get_public_key_short(&self) -> Result<&str, CredentialError> {
        make_public_key_short(&self.public_key_str)
    }
}
