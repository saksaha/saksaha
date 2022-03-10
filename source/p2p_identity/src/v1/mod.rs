pub use k256::{
    ecdh::EphemeralSecret,
    ecdsa::{
        signature::{Signer, Verifier},
        Signature, SigningKey, VerifyingKey,
    },
    elliptic_curve::sec1::ToEncodedPoint,
    EncodedPoint, PublicKey, SecretKey,
};

pub type PeerId = [u8; PUBLIC_KEY_LEN];

pub const PUBLIC_KEY_LEN: usize = 65;
pub const SAKSAHA: &[u8; 7] = b"saksaha";

pub struct Identity {
    pub secret_key: SecretKey,
    pub public_key: PeerId,
    pub peer_id: String,
    pub sig: Signature,
}

impl Identity {
    pub fn new(secret: String, peer_id: String) -> Result<Identity, String> {
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

        let public_key: PeerId = {
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
            let p = crypto::encode_hex(&public_key);
            if p != peer_id {
                return Err(format!(
                    "public key built from bytes differ \
                    from the one in pconfig"
                ));
            }
        }

        let sig = {
            let signing_key = SigningKey::from(&secret_key);
            let sig = crypto::make_signature(signing_key, SAKSAHA);
            sig
        };

        let credential = Identity {
            secret_key,
            public_key,
            peer_id,
            sig,
        };

        Ok(credential)
    }
}
