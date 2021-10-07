use tokio::task::JoinError;

use super::errorkind::ErrorKind;
use std::fmt;

#[derive(PartialEq)]
pub struct Error {
    kind: ErrorKind,
    msg: String,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: String) -> Error {
        return Error { kind, msg };
    }

    pub fn new_default(msg: String) -> Error {
        return Error {
            kind: ErrorKind::Default,
            msg,
        };
    }

    pub fn kind(&self) -> ErrorKind {
        return self.kind;
    }
}

impl From<JoinError> for Error {
    fn from(err: JoinError) -> Error {
        return Error::new(ErrorKind::Default, err.to_string());
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        return Error::new(ErrorKind::Default, err.to_string());
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

#[macro_export]
macro_rules! err {
    ($str: expr) => {
        {
            Err($crate::common::Error::new($crate::common::ErrorKind::Default,
                format!($str)))
        }
    };

    ($str_format: expr, $($arg:tt)*) => {
        {
            let msg = format!("{}", format_args!($str_format, $($arg)*));
            Err($crate::common::Error::new(
                $crate::common::ErrorKind::Default, msg))
        }
    };
}

// deprecated
#[macro_export]
macro_rules! err_with_kind {
    ($err_kind: expr, $msg: expr) => {{
        Err(Error::new($err_kind, format!($msg)))
    }};

    ($err_kind: expr, $str_format: expr, $($arg:tt)*) => {{
        let msg = format!("{}", format_args!($str_format, $($arg)*));
        Err($crate::common::Error::new($err_kind, msg))
    }};
}
