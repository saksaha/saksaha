mod apis;
mod constants;
mod ctr_fn;
mod utils;
mod vm;
pub(crate) mod wasm_bootstrap;

#[cfg(test)]
mod tests;

pub use apis::*;
pub(crate) use constants::*;
pub use ctr_fn::*;
pub use utils::*;
pub use vm::*;

pub(crate) type VMError = Box<dyn std::error::Error + Send + Sync>;

pub const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];
