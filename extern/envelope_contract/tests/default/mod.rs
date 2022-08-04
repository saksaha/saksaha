mod constants;
mod test_a;
mod test_messenger;
mod test_msg_multi_clients;

pub use constants::*;

pub(crate) const DUMMY_CHANNEL_ID_1: &str = "ch_12";
pub(crate) const ENVELOPE_CTR_ADDR: &'static str = "envelope_contract_addr";
pub(crate) const ARG_SERIALIZED_INPUT: &str = "serialized_input";
pub(crate) const ARG_CH_ID: &str = "ch_id";
