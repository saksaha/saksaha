use k256::SecretKey;

use crate::{common::SakResult, crypto::Crypto, err_res};

pub struct Credential {
    pub secret_key: SecretKey,
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

        let credential = Credential {
            secret_key,
        };

        Ok(credential)
    }
}
