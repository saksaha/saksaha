pub const STORAGE_CAP: usize = 100;

pub const DUMMY_CHANNEL_ID_1: &str = "dummy_channel_1";

pub const DUMMY_CHANNEL_ID_2: &str = "dummy_channel_2";

pub const ENVELOPE_CONTRACT: &[u8] = include_bytes!(
    "../../../../source/prebuild/\
        envelope_contract.postprocess.wasm"
);

pub(crate) const ARG_CH_ID: &str = "ch_id";
pub(crate) const ARG_CIPHER_TEXT: &str = "cipher_text";
pub(crate) const ARG_EPH_PK: &str = "eph_pk";
pub(crate) const ARG_SRC_PK: &str = "src_pk";
pub(crate) const ARG_DST_PK: &str = "dst_pk";
pub(crate) const ARG_SERIALIZED_INPUT: &str = "serialized_input";
