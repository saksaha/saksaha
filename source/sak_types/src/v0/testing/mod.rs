mod block;
mod tx;
mod values;

pub use block::*;
pub use tx::*;

pub const VALIDATOR: &[u8] =
    include_bytes!("../../../../prebuild/sak_validator.postprocess.wasm");

pub const VALIDATOR_CTR_ADDR: &'static str = "test_validator_1";
