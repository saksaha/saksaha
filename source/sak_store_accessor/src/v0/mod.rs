mod accessor;

#[cfg(test)]
mod tests;

pub use accessor::*;

pub(crate) type StoreAccessorError = Box<dyn std::error::Error + Send + Sync>;
