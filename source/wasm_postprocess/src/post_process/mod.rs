mod multi_returns;
mod optimize;

pub use multi_returns::*;
pub use optimize::*;

pub type PostProcessError = Box<dyn std::error::Error + Send + Sync>;
