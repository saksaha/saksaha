use crypto::Crypto;
use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};
use saksaha_discovery::identity::Identity;
use crate::{common::Result, err};

pub struct Credential {
    pub secret_key: SecretKey,
    pub public_key: String,
    pub public_key_bytes: [u8; 65],
}

impl Credential {
    pub fn new(secret: String, public_key: String) -> Result<Credential> {
        let secret_bytes = match Crypto::decode_hex(secret.to_owned()) {
            Ok(v) => v,
            Err(err) => {
                return err!("Error making secret key, err: {}", err);
            }
        };

        let secret_key = match SecretKey::from_bytes(secret_bytes) {
            Ok(sk) => sk,
            Err(err) => {
                return err!(
                    "Error creating SecretKey object, err: {}",
                    err
                );
            }
        };

        let public_key_bytes: [u8; 65] = {
            let b = secret_key.public_key().to_encoded_point(false).to_bytes();

            if b.len() != 65 {
                return err!(
                    "Error encoding public key into bytes, size does not fit"
                );
            }

            let mut buf = [0; 65];
            buf.clone_from_slice(&b);
            buf
        };

        {
            let p = Crypto::encode_hex(&public_key_bytes);
            if p != public_key {
                return err!(
                    "public key built from bytes differ \
                    from the one in pconfig"
                );
            }
        }

        let credential = Credential {
            secret_key,
            public_key,
            public_key_bytes,
        };

        Ok(credential)
    }
}

impl Identity for Credential {
    fn public_key() {

    }
}
