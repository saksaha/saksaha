use crate::common::Error;

#[derive(Debug, Clone)]
pub struct Msg {
    pub label: String,
    pub kind: Kind,
}

impl Msg {
    pub fn new(label: String, kind: Kind) -> Msg {
        Msg { label, kind }
    }
}

impl From<Msg> for Error {
    fn from(m: Msg) -> Error {
        let err = Error::new(crate::common::ErrorKind::Default, "".into());
        err
    }
}

#[derive(Clone, Debug)]
pub enum Kind {
    Default,

    SetupFailure,

    ResourceNotAvailable,
}
