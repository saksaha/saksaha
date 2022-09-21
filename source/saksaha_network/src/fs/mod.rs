mod paths;

pub(crate) use paths::*;

pub type SaksahaFSError = Box<dyn std::error::Error + Send + Sync>;
