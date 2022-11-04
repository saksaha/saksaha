// pub(crate) mod keys {
//     pub const SINGLETON: &[u8; 1] = &[0];
// }

pub(crate) mod cfs {
    pub const TX_HASH_BY_CTR_ADDR: &str = "tx_hash_by_ctr_addr";

    pub const TX_HASH_BY_SN: &str = "tx_hash_by_sn";

    pub const TX_TYPE: &str = "tx_type";

    pub const DATA: &str = "data";

    pub const CM_IDX: &str = "cm_by_cm_idx";

    pub const CM_IDX_CM: &str = "cm_idx_by_cm";

    pub const BLOCK_MERKLE_RT: &str = "block_merkle_rt";

    pub const EMPTY_VALUE: &str = "empty_value";

    pub const MERKLE_NODE: &str = "merkle_node";

    pub const BLOCK_HASH: &str = "block_hash";

    pub const CTR_STATE: &str = "ctr_state";

    // test
    pub const MINT_TX_ENTITY: &str = "mint_tx_entity";
    pub const POUR_TX_ENTITY: &str = "pour_tx_entity";
    pub const BLOCK_ENTITY: &str = "block_entity";
}

pub enum LedgerCols {
    TxHashByCtrAddr,
    TxHashBySN,
    TxType,
    Data,
    CMByCMIdx,
    CMIdxByCM,
    BlockMerkleRt,
    EmptyValue,
    MerkleNode,
    BlockHash,
    CtrState,

    // test
    MintTxEntity,
    PourTxEntity,
    BlockEntity,
}

impl LedgerCols {
    pub fn as_str(&self) -> &'static str {
        match self {
            LedgerCols::TxHashByCtrAddr => "tx_hash_by_ctr_addr",
            LedgerCols::TxHashBySN => "tx_hash_by_sn",
            LedgerCols::TxType => "tx_type",
            LedgerCols::Data => "data",
            LedgerCols::CMByCMIdx => "cm_by_cm_idx",
            LedgerCols::CMIdxByCM => "cm_idx_by_cm",
            LedgerCols::BlockMerkleRt => "block_merkle_rt",
            LedgerCols::EmptyValue => "empty_value",
            LedgerCols::MerkleNode => "merkle_node",
            LedgerCols::BlockHash => "block_hash",
            LedgerCols::CtrState => "ctr_state",
            LedgerCols::MintTxEntity => "mint_tx_entity",
            LedgerCols::PourTxEntity => "pour_tx_entity",
            LedgerCols::BlockEntity => "block_entity",
        }
    }
}
