use super::utils;
use crate::{
    Cm, CmIdx, MintTx, PourTx, Tx, TxCtrOp, TxType, TypesError,
    WASM_MAGIC_NUMBER,
};
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

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

    pub fn get_cms(&self) -> Vec<Cm> {
        match &self {
            TxCandidate::Mint(c) => c.get_cms(),
            TxCandidate::Pour(c) => c.get_cms(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MintTxCandidate {
    //
    pub created_at: String,

    //
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,

    //
    pub author_sig: String,

    //
    pub ctr_addr: String,

    //
    pub cm: U8Arr32,

    //
    pub v: U8Arr32,

    //
    pub k: U8Arr32,

    //
    pub s: U8Arr32,

    //
    tx_hash: String,
}

impl MintTxCandidate {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cm: U8Arr32,
        v: U8Arr32,
        k: U8Arr32,
        s: U8Arr32,
    ) -> MintTxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));

        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_bytes(),
        ];

        let tx_hash = sak_crypto::compute_hash(&hashable_items);

        MintTxCandidate {
            created_at,
            data,
            author_sig,
            ctr_addr,
            cm,
            v,
            k,
            s,
            tx_hash,
        }
    }

    pub fn get_tx_type(&self) -> TxType {
        TxType::Mint
    }

    pub fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        utils::get_ctr_op(&self.ctr_addr, &self.data)
    }

    pub fn get_cms(&self) -> Vec<Cm> {
        vec![self.cm]
    }

    pub fn upgrade(
        self,
        // tx_height: u128,
        cm_idx_1: CmIdx,
    ) -> Tx {
        Tx::Mint(MintTx::new(
            self, // tx_height,
            cm_idx_1,
        ))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PourTxCandidate {
    //
    pub created_at: String,

    //
    #[serde(with = "serde_bytes")]
    pub data: Vec<u8>,

    //
    pub author_sig: String,

    //
    pub ctr_addr: String,

    //
    pub pi: Vec<u8>,

    //
    pub sn_1: U8Arr32,

    //
    // pub sn_2: U8Arr32,

    //
    pub cm_1: U8Arr32,

    //
    pub cm_2: U8Arr32,

    //
    pub merkle_rt: U8Arr32,

    //
    tx_hash: String,
}

impl PourTxCandidate {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        pi: Vec<u8>,
        sn_1: U8Arr32,
        // sn_2: U8Arr32,
        cm_1: U8Arr32,
        cm_2: U8Arr32,
        merkle_rt: U8Arr32,
    ) -> PourTxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));

        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_bytes(),
            &pi,
        ];

        let tx_hash = sak_crypto::compute_hash(&hashable_items);

        PourTxCandidate {
            created_at,
            data,
            author_sig,
            ctr_addr,
            pi,
            sn_1,
            // sn_2,
            cm_1,
            cm_2,
            merkle_rt,
            tx_hash,
        }
    }

    pub fn upgrade(
        self,
        // tx_height: u128
        cm_idx: CmIdx,
        // cm_idx_2: CmIdx,
    ) -> Tx {
        let later_dynamically_determined_cm_idx_2 = cm_idx + 1;

        Tx::Pour(PourTx::new(
            self,
            cm_idx,
            later_dynamically_determined_cm_idx_2,
            // tx_height
        ))
    }

    pub fn get_tx_type(&self) -> TxType {
        TxType::Pour
    }

    pub fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        utils::get_ctr_op(&self.ctr_addr, &self.data)
    }

    pub fn get_cms(&self) -> Vec<Cm> {
        vec![self.cm_1, self.cm_2]
    }
}
