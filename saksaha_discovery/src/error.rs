use tokio::task::JoinError;
use std::fmt;

#[derive(PartialEq)]
pub struct Error {
    msg: String,
}

impl Error {
    pub fn new(msg: String) -> Error {
        return Error { msg };
    }

    pub fn new_default(msg: String) -> Error {
        return Error {
            msg,
        };
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Error {
        return Error::new(err.to_string());
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        return Error::new(err.to_string());
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.msg);
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return write!(f, "{}", self.msg);
    }
}
