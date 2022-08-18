use crate::EnvelopeTypeError;
use serde::{Deserialize, Serialize};

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
    ) -> Result<Channel, EnvelopeTypeError> {
        let open_ch = Channel {
            ch_id,
            eph_key,
            sig,
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
