pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};

// pub type PeerId = [u8; PUBLIC_KEY_LEN];

// 64 + 1 (flag for whether the key is compressed or not)
pub const PUBLIC_KEY_LEN: usize = 64 + 1;
pub const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct P2PIdentity {
    pub secret: String,
    pub public_key_str: String,
    pub secret_key: SecretKey,
    pub public_key_bytes: [u8; PUBLIC_KEY_LEN],
    pub sig: Signature,
}

impl P2PIdentity {
    pub fn new(
        secret: String,
        public_key_str: String,
    ) -> Result<P2PIdentity, String> {
        let secret_bytes = match crypto::decode_hex(secret.to_owned()) {
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

        let public_key_bytes: [u8; PUBLIC_KEY_LEN] = {
            let b = secret_key.public_key().to_encoded_point(false).to_bytes();

            if b.len() != 65 {
                return Err(format!(
                    "Error encoding public key into bytes, size does not fit"
                ));
            }

            let mut buf = [0; 65];
            buf.clone_from_slice(&b);

            let pk_encoded = crypto::encode_hex(&b);
            if pk_encoded != public_key_str {
                return Err(format!(
                    "Encoded public key is different from the restored one",
                ));
            }

            buf
        };

        let sig = {
            let signing_key = SigningKey::from(&secret_key);
            let sig = crypto::make_signature(signing_key, SAKSAHA);
            sig
        };

        let credential = P2PIdentity {
            secret,
            secret_key,
            public_key_str,
            public_key_bytes,
            sig,
        };

        Ok(credential)
    }
}
