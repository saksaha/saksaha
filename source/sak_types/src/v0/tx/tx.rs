use crate::{MintTxCandidate, PourTxCandidate, TypesError};
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
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MintTx {
    //
    pub tx_candidate: MintTxCandidate,

    //
    pub tx_height: u128,
}

impl MintTx {
    pub fn new(tx_candidate: MintTxCandidate, tx_height: u128) -> MintTx {
        MintTx {
            tx_candidate,
            tx_height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PourTx {
    //
    pub tx_candidate: PourTxCandidate,

    //
    pub tx_height: u128,
}

impl PourTx {
    pub fn new(tx_candidate: PourTxCandidate, tx_height: u128) -> PourTx {
        PourTx {
            tx_candidate,
            tx_height,
        }
    }
}

pub struct ContractCallData {
    pub fn_name: String,
    pub args: Vec<Vec<u8>>,
}

pub enum TxCtrOp {
    ContractCall,
    ContractDeploy,
    None,
}
