use crate::EnvelopeTypeError;
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}

impl Channel {
    pub fn new(
        ch_id: String,
        eph_key: String,
        sig: String,
        key: U8Arr32,
    ) -> Result<Channel, EnvelopeTypeError> {
        let ch_id_enc = {
            let ch_id_enc = sak_crypto::aes_encrypt(&key, &ch_id.as_bytes())?;

            serde_json::to_string(&ch_id_enc)?
        };

        let sig_enc = {
            let sig_enc = sak_crypto::aes_encrypt(&key, &sig.as_bytes())?;

            serde_json::to_string(&sig_enc)?
        };

        let open_ch = Channel {
            ch_id: ch_id_enc,
            eph_key,
            sig: sig_enc,
        };

        Ok(open_ch)
    }

    pub fn default() -> Channel {
        Channel {
            ch_id: String::default(),
            eph_key: String::default(),
            sig: String::default(),
        }
    }
}
