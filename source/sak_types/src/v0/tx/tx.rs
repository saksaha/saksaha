use super::CmIdx;
use crate::{
    Cm, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Sn, TxCandidate,
};
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

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
            Tx::Mint(t) => t.tx_candidate.cms.len(),
            Tx::Pour(t) => t.tx_candidate.cms.len(),
        }
    }

    pub fn downgrade(self) -> TxCandidate {
        match self {
            Tx::Mint(t) => TxCandidate::Mint(t.downgrade()),
            Tx::Pour(t) => TxCandidate::Pour(t.downgrade()),
        }
    }

    pub fn get_cm_pairs(&self) -> Vec<(&CmIdx, &Cm)> {
        match self {
            Tx::Mint(t) => t.get_cm_pairs(),
            Tx::Pour(t) => t.get_cm_pairs(),
        }
    }

    pub fn get_sn(&self) -> Sn {
        match self {
            Tx::Mint(_t) => U8Arr32::default(),
            Tx::Pour(t) => t.get_sn(),
        }
    }
}

impl std::fmt::Display for Tx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Tx::Pour(t) => write!(f, "{}", t),
            Tx::Mint(t) => write!(f, "{}", t),
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
