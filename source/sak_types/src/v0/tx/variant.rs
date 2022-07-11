use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub enum TxCandidateVariant {
    Mint(MintTxCandidate),
    Pour(PourTxCandidate),
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct MintTxCandidate {
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    //
    author_sig: String,

    //
    ctr_addr: Option<String>,

    pub cm: Vec<u8>,
    pub v: String,
    pub k: String,
    pub s: String,

    tx_hash: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct PourTxCandidate {
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    //
    author_sig: String,

    //
    ctr_addr: Option<String>,

    pub pi: Vec<u8>,
    pub sn_1: String,
    pub sn_2: String,
    pub cm_1: Vec<u8>,
    pub cm_2: Vec<u8>,
    pub merkle_rt: Vec<u8>,

    tx_hash: String,
}

impl TxCandidateVariant {
    pub(crate) fn get_hashable_items(&self) -> Vec<&[u8]> {
        vec![]
    }
}
