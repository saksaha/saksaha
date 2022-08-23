use log::warn;
use sak_crypto::{self, SakKey, SecretKey, SigningKey, ToEncodedPoint};
use sak_p2p_id::Credential as P2PCredential;

use crate::EnvelopeError;

pub(crate) struct Credential {
    pub public_key: String,
    pub secret: String,
    pub acc_addr: String,
    pub signature: String,
}

impl Credential {
    pub fn new(
        public_key: Option<String>,
        secret: Option<String>,
    ) -> Result<Credential, EnvelopeError> {
        let (public_key, secret, acc_addr, secret_key) =
            match (public_key, secret) {
                (Some(public_key), Some(secret)) => {
                    let credential = P2PCredential::new(&secret, &public_key)?;
                    let acc_addr =
                        SakKey::create_acc_addr(&credential.public_key);

                    let secret_bytes = sak_crypto::decode_hex(&secret)?;

                    let secret_key = SecretKey::from_bytes(secret_bytes)?;

                    (public_key, secret, acc_addr, secret_key)
                }
                _ => {
                    let (sk, pk) = SakKey::generate();
                    let acc_addr = SakKey::create_acc_addr(&pk);

                    let secret_str = sak_crypto::encode_hex(&sk.to_bytes());

                    let public_key_str = sak_crypto::encode_hex(
                        &pk.to_encoded_point(false).to_bytes(),
                    );

                    (public_key_str, secret_str, acc_addr, sk)
                }
            };

        let signature = {
            let sign_key = SigningKey::from(&secret_key);
            let sign_key_vec = sign_key.to_bytes().to_vec();
            match serde_json::to_string(&sign_key_vec) {
                Ok(str) => str,
                Err(err) => {
                    return Err(format!(
                        "Failed to change vec to string, err: {}",
                        err
                    )
                    .into());
                }
            }
        };

        Ok(Credential {
            public_key,
            secret,
            acc_addr,
            signature,
        })
    }
}
