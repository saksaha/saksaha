mod vm;
mod wasm;

#[cfg(test)]
mod tests;

pub use vm::*;

pub(crate) type VMError = Box<dyn std::error::Error + Send + Sync>;
