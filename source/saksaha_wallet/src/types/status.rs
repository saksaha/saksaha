#[repr(u64)]
#[derive(Debug)]
pub(crate) enum Status {
    Unused = 0,
    Used,
}

impl std::fmt::Display for Status {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Used => "Used".fmt(fmt),
            Status::Unused => "Unused".fmt(fmt),
        }
    }
}
