mod macros;
mod request;

pub use request::*;
use std::collections::HashMap;
pub use std::error::Error;

pub(crate) type BoxedError = Box<dyn std::error::Error + Send + Sync>;

pub type Storage = HashMap<String, String>;
