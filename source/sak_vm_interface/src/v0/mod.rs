mod interface;
mod receipt;
mod state;

pub use interface::*;
pub use receipt::*;
pub use state::*;

pub type VMInterfaceError = Box<dyn std::error::Error + Send + Sync>;
