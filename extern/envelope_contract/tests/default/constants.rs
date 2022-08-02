pub const STORAGE_CAP: usize = 100;

pub const INIT_CHANNEL_ID_1: &str = "ch_12";

pub const DUMMY_CHANNEL_ID_1: &str = "dummy_channel_1";

pub const DUMMY_CHANNEL_ID_2: &str = "dummy_channel_2";

pub const DUMMY_CHANNEL_ID_3: &str = "dummy_channel_3";

pub const ENVELOPE_CONTRACT: &[u8] = include_bytes!(
    "../../../../source/prebuild/\
        envelope_contract.postprocess.wasm"
);

pub const ARG_CH_ID: &str = "ch_id";

pub const ARG_DST_PK: &str = "dst_pk";

pub const ARG_SERIALIZED_INPUT: &str = "serialized_input";

// pub const ARG_CIPHER_TEXT: &str = "cipher_text";
// pub const ARG_EPH_PK: &str = "eph_pk";
// pub const ARG_SRC_PK: &str = "src_pk";
