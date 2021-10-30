use crypto::{Crypto, Signature, SigningKey};
use k256::{elliptic_curve::sec1::ToEncodedPoint, SecretKey};
use saksaha_discovery::identity::Identity;

pub const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct Credential {
    pub secret_key: SecretKey,
    pub public_key_str: String,
    pub public_key_bytes: [u8; 65],
    sig: Signature,
}

impl Credential {
    pub fn new(
        secret: String,
        public_key_str: String,
    ) -> Result<Credential, String> {
        let secret_bytes = match Crypto::decode_hex(secret.to_owned()) {
            Ok(v) => v,
            Err(err) => {
                return Err(format!("Error making secret key, err: {}", err));
            }
        };

        let secret_key = match SecretKey::from_bytes(secret_bytes) {
            Ok(sk) => sk,
            Err(err) => {
                return Err(format!(
                    "Error creating SecretKey object, err: {}",
                    err
                ));
            }
        };

        let public_key_bytes: [u8; 65] = {
            let b = secret_key.public_key().to_encoded_point(false).to_bytes();

            if b.len() != 65 {
                return Err(format!(
                    "Error encoding public key into bytes, size does not fit"
                ));
            }

            let mut buf = [0; 65];
            buf.clone_from_slice(&b);
            buf
        };

        {
            let p = Crypto::encode_hex(&public_key_bytes);
            if p != public_key_str {
                return Err(format!(
                    "public key built from bytes differ \
                    from the one in pconfig"
                ));
            }
        }

        let sig = {
            let signing_key = SigningKey::from(&secret_key);
            let sig = Crypto::make_sign(signing_key, SAKSAHA);
            sig
        };

        let credential = Credential {
            secret_key,
            public_key_str,
            public_key_bytes,
            sig,
        };

        Ok(credential)
    }
}

impl Identity for Credential {
    fn public_key_bytes(&self) -> [u8; 65] {
        self.public_key_bytes
    }

    fn secret_key(&self) -> &SecretKey {
        &self.secret_key
    }

    fn sig(&self) -> Signature {
        self.sig
    }
}
