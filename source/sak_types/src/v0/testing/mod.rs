mod block;
mod coin;
mod tx;
mod values;

pub use block::*;
pub use coin::*;
pub use tx::*;

pub const VALIDATOR: &[u8] =
    include_bytes!("../../../../prebuild/sak_validator.postprocess.wasm");

pub const VALIDATOR_CTR_ADDR: &'static str = "test_validator_1";

pub(crate) const VALIDATOR_SIG: &str = "validator_sig";

pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";

pub(crate) const ENVELOPE: &[u8] =
    include_bytes!("../../../../prebuild/envelope_contract.postprocess.wasm");
