use crate::WalletError;

#[derive(Debug, PartialEq)]
pub(crate) enum Status {
    Unused,
    Used,
}

impl std::fmt::Display for Status {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Unused => "Unused".fmt(fmt),
            Status::Used => "Used".fmt(fmt),
        }
    }
}

impl AsRef<[u8]> for Status {
    fn as_ref(&self) -> &[u8] {
        match self {
            Status::Unused => "Unused".as_ref(),
            Status::Used => "Used".as_ref(),
        }
    }
}

impl Status {
    pub fn from_u8(v: Vec<u8>) -> Result<Status, WalletError> {
        if v == "Unused".as_bytes().to_vec() {
            return Ok(Status::Unused);
        } else if v == "Used".as_bytes().to_vec() {
            return Ok(Status::Used);
        } else {
            return Err(format!("Invalid Vec<u8> to convert into Status").into());
        }
    }
}
