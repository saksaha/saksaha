use crate::{TxHash, TypesError};

#[derive(Debug, PartialEq, Clone)]
pub enum CoinStatus {
    Unconfirmed(Option<TxHash>),
    Unused,
    Used,
}

impl std::fmt::Display for CoinStatus {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Unconfirmed(Some(TxHash)) => {
                "Unconfirmed, tx_hash: Some".fmt(fmt)
            }
            Self::Unconfirmed(None) => "Unconfirmed, tx_hash: None".fmt(fmt),
            Self::Unused => "Unused".fmt(fmt),
            Self::Used => "Used".fmt(fmt),
        }
    }
}

impl AsRef<[u8]> for CoinStatus {
    fn as_ref(&self) -> &[u8] {
        match self {
            Self::Unconfirmed => "Unconfirmed".fmt(fmt),
            Self::Unused => "Unused".as_ref(),
            Self::Used => "Used".as_ref(),
        }
    }
}

impl CoinStatus {
    pub fn from_u8(v: Vec<u8>) -> Result<CoinStatus, TypesError> {
        if v == "Unused".as_bytes().to_vec() {
            return Ok(Self::Unused);
        } else if v == "Used".as_bytes().to_vec() {
            return Ok(Self::Used);
        } else {
            return Err(
                format!("Invalid Vec<u8> to convert into Status").into()
            );
        }
    }
}
