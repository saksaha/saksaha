use super::CmIdx;
use crate::{MintTxCandidate, PourTxCandidate};
use serde::{Deserialize, Serialize};

pub const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];

#[derive(Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Tx {
    Mint(MintTx),
    Pour(PourTx),
}

impl Tx {
    pub fn get_tx_hash(&self) -> &String {
        match self {
            Tx::Mint(t) => t.tx_candidate.get_tx_hash(),
            Tx::Pour(t) => t.tx_candidate.get_tx_hash(),
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        match &self {
            Tx::Mint(t) => &t.tx_candidate.data,
            Tx::Pour(t) => &t.tx_candidate.data,
        }
    }

    pub fn get_cm_count(&self) -> usize {
        match &self {
            Tx::Mint(t) => [&t.tx_candidate.cm].len(),
            Tx::Pour(t) => [&t.tx_candidate.cm_1, &t.tx_candidate.cm_2].len(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MintTx {
    //
    pub tx_candidate: MintTxCandidate,

    //
    // pub tx_height: u128,
    pub cm_idx_1: CmIdx,
}

impl MintTx {
    pub fn new(
        tx_candidate: MintTxCandidate,
        // tx_height: u128,
        cm_idx_1: CmIdx,
    ) -> MintTx {
        MintTx {
            tx_candidate,
            // tx_height,
            cm_idx_1,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PourTx {
    //
    pub tx_candidate: PourTxCandidate,
    pub cm_idx_1: CmIdx,
    pub cm_idx_2: CmIdx,
    // pub tx_height: u128,
}

impl PourTx {
    pub fn new(
        tx_candidate: PourTxCandidate,
        // tx_height: u128
        cm_idx_1: CmIdx,
        cm_idx_2: CmIdx,
    ) -> PourTx {
        PourTx {
            tx_candidate,
            cm_idx_1,
            cm_idx_2,
            // tx_height,
        }
    }
}

pub struct ContractCallData {
    pub fn_name: String,
    pub args: Vec<Vec<u8>>,
}

#[derive(Debug)]
pub enum TxCtrOp {
    ContractCall,
    ContractDeploy,
    None,
}
