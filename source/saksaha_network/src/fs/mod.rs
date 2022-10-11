mod fs;

pub use fs::*;

pub type SaksahaFSError = Box<dyn std::error::Error + Send + Sync>;
