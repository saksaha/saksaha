mod fs;
mod paths;

pub use fs::*;
pub use paths::*;

pub type FSError = Box<dyn std::error::Error + Send + Sync>;
