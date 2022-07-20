pub(crate) mod keys {
    pub const SINGLETON: &[u8; 1] = &[0];
}

pub(crate) mod cfs {
    pub const TX_HASH: &str = "tx_hash";

    pub const TX_TYPE: &str = "tx_type";

    pub const PI: &str = "pi";

    pub const AUTHOR_SIG: &str = "author_sig";

    pub const TX_CREATED_AT: &str = "tx_created_at";

    pub const BLOCK_CREATED_AT: &str = "block_created_at";

    pub const DATA: &str = "data";

    pub const CTR_ADDR: &str = "ctr_addr";

    pub const TX_HEIGHT: &str = "tx_height";

    pub const CM: &str = "cm";

    pub const V: &str = "v";

    pub const K: &str = "k";

    pub const S: &str = "s";

    pub const SN_1: &str = "sn_1";

    pub const SN_2: &str = "sn_2";

    pub const CM_1: &str = "cm_1";

    pub const CM_2: &str = "cm_2";

    pub const BLOCK_CM_COUNT: &str = "block_cm_count";

    pub const LEDGER_CM_COUNT: &str = "ledger_cm_count";

    pub const BLOCK_MERKLE_RT: &str = "block_merkle_rt";

    pub const PRF_MERKLE_RT: &str = "prf_merkle_rt";

    pub const MERKLE_NODE: &str = "merkle_node";

    pub const VALIDATOR_SIG: &str = "validator_sig";

    pub const TX_HASHES: &str = "tx_hashes";

    pub const WITNESS_SIGS: &str = "witness_sigs";

    pub const BLOCK_HEIGHT: &str = "block_height";

    pub const BLOCK_HASH: &str = "block_hash";

    pub const CTR_STATE: &str = "ctr_state";
}
