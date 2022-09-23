mod identity;
mod utils;

pub use identity::Identity;
pub use sak_credential::make_public_key_short;
pub use utils::*;

pub type IdentityError = Box<dyn std::error::Error + Send + Sync>;
