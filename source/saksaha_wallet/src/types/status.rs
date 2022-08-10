#[derive(Debug)]
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
