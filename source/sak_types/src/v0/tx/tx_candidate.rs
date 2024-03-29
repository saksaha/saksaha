use crate::{Cm, CmIdx, MintTxCandidate, PourTxCandidate, Tx, TxCtrOp, TypesError};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum TxCandidate {
    Mint(MintTxCandidate),
    Pour(PourTxCandidate),
}

impl TxCandidate {
    pub fn upgrade(self, cm_idx: CmIdx) -> Tx {
        match self {
            TxCandidate::Mint(c) => c.upgrade(cm_idx),
            TxCandidate::Pour(c) => c.upgrade(cm_idx),
        }
    }

    pub fn into_mint_tx_candidate(self) -> Result<MintTxCandidate, TypesError> {
        match self {
            TxCandidate::Mint(c) => Ok(c),
            TxCandidate::Pour(_) => Err(format!("tx candidate is not pour candidate").into()),
        }
    }

    pub fn into_pour_tx_candidate(self) -> Result<PourTxCandidate, TypesError> {
        match self {
            TxCandidate::Pour(c) => Ok(c),
            TxCandidate::Mint(_) => Err(format!("tx candidate is not mint candidate").into()),
        }
    }

    pub fn get_cm_count(&self) -> u128 {
        match self {
            TxCandidate::Mint(_) => 1,
            TxCandidate::Pour(_) => 2,
        }
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        match self {
            TxCandidate::Mint(c) => c.get_ctr_op(),
            TxCandidate::Pour(c) => c.get_ctr_op(),
        }
    }

    pub fn get_ctr_addr(&self) -> &String {
        match &self {
            TxCandidate::Mint(c) => &c.ctr_addr,
            TxCandidate::Pour(c) => &c.ctr_addr,
        }
    }

    pub fn get_data(&self) -> &Vec<u8> {
        match &self {
            TxCandidate::Mint(c) => &c.data,
            TxCandidate::Pour(c) => &c.data,
        }
    }

    pub fn get_tx_hash(&self) -> &String {
        match &self {
            TxCandidate::Mint(c) => c.get_tx_hash(),
            TxCandidate::Pour(c) => c.get_tx_hash(),
        }
    }

    pub fn get_cms(&self) -> &Vec<Cm> {
        match &self {
            TxCandidate::Mint(c) => c.get_cms(),
            TxCandidate::Pour(c) => c.get_cms(),
        }
    }

    pub fn get_created_at(&self) -> &String {
        match &self {
            TxCandidate::Mint(c) => &c.created_at,
            TxCandidate::Pour(c) => &c.created_at,
        }
    }

    pub fn get_author_sig(&self) -> &String {
        match &self {
            TxCandidate::Mint(c) => &c.author_sig,
            TxCandidate::Pour(c) => &c.author_sig,
        }
    }
}
