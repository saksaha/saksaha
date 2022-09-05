pub(crate) mod keys {
    pub const SINGLETON: &[u8; 1] = &[0];
}

pub(crate) mod cfs {
    pub const TX_HASH_BY_CTR_ADDR: &str = "tx_hash_by_ctr_addr";

    pub const TX_HASH_BY_SN: &str = "tx_hash_by_sn";

    pub const TX_TYPE: &str = "tx_type";

    pub const PI: &str = "pi";

    pub const AUTHOR_SIG: &str = "author_sig";

    pub const TX_CREATED_AT: &str = "tx_created_at";

    pub const BLOCK_CREATED_AT: &str = "block_created_at";

    pub const DATA: &str = "data";

    pub const CTR_ADDR: &str = "ctr_addr";

    pub const CM_IDX: &str = "cm_idx";

    pub const CM_IDX_CM: &str = "cm_idx_cm";

    pub const V: &str = "v";

    pub const K: &str = "k";

    pub const S: &str = "s";

    pub const SNS: &str = "sns";

    pub const CMS: &str = "cms";

    pub const CM_COUNT: &str = "cm_count";

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
