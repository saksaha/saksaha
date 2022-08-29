use crate::EnvelopeError;
use sak_crypto::{self, PublicKey, SakKey, SecretKey, ToEncodedPoint};
use sak_p2p_id::Credential as P2PCredential;

pub(crate) struct Credential {
    pub public_key: PublicKey,
    pub secret_key: SecretKey,
    pub public_key_str: String,
    pub secret_key_str: String,
    pub acc_addr: String,
}

impl Credential {
    pub fn new(
        public_key: Option<String>,
        secret: Option<String>,
    ) -> Result<Credential, EnvelopeError> {
        match (public_key, secret) {
            (Some(public_key), Some(secret)) => {
                let p2p_credential = P2PCredential::new(&secret, &public_key)?;
                let acc_addr =
                    SakKey::create_acc_addr(&p2p_credential.public_key);

                let c = Credential {
                    public_key: p2p_credential.public_key,
                    secret_key: p2p_credential.secret_key,
                    public_key_str: p2p_credential.public_key_str,
                    secret_key_str: p2p_credential.secret,
                    acc_addr,
                };

                return Ok(c);
            }
            _ => {
                let c = Credential::new_random()?;

                return Ok(c);
            }
        }
    }

    pub fn new_random() -> Result<Credential, EnvelopeError> {
        let (secret_key, public_key) = SakKey::generate();
        let acc_addr = SakKey::create_acc_addr(&public_key);

        let public_key_str = sak_crypto::encode_hex(
            &public_key.to_encoded_point(false).as_bytes(),
        );
        let secret_key_str =
            sak_crypto::encode_hex(&secret_key.to_bytes() as &[u8]);

        let c = Credential {
            public_key,
            secret_key,
            public_key_str,
            secret_key_str,
            acc_addr,
        };

        Ok(c)
    }

    pub fn sign(&self) -> String {
        let ret = format!("{}-sig", self.public_key_str);
        ret
    }
}
