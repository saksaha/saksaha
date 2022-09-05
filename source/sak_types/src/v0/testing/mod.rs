mod block;
mod coin;
mod tx;
mod values;

pub use block::*;
pub use coin::*;
pub use tx::*;
pub use values::*;

pub const VALIDATOR: &[u8] =
    include_bytes!("../../../../prebuild/sak_validator.postprocess.wasm");

// pub const VALIDATOR_CTR_ADDR: &'static str = "test_validator_1";
pub(crate) const VALIDATOR_CTR_ADDR: &str = "validator_contract_addr";
