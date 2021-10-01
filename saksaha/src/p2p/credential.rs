use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};

use crate::{common::SakResult, crypto::Crypto, err_res};

pub struct Credential {
    pub secret_key: SecretKey,
    pub public_key_bytes: [u8; 65],
}

impl Credential {
    pub fn new(secret: String) -> SakResult<Credential> {
        let secret_bytes = match Crypto::decode_hex(secret.to_owned()) {
            Ok(v) => v,
            Err(err) => {
                return err_res!("Error making secret key, err: {}", err);
            }
        };

        let secret_key = match SecretKey::from_bytes(secret_bytes) {
            Ok(sk) => sk,
            Err(err) => {
                return err_res!(
                    "Error creating SecretKey object, err: {}",
                    err
                );
            }
        };

        let public_key_bytes: [u8; 65] = {
            let b = secret_key.public_key().to_encoded_point(false).to_bytes();

            if b.len() != 65 {
                return err_res!(
                    "Error encoding public key into bytes, size does not fit"
                );
            }

            let mut buf = [0; 65];
            buf.clone_from_slice(&b);
            buf
        };

        let credential = Credential {
            secret_key,
            public_key_bytes,
        };

        Ok(credential)
    }
}
