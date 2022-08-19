use super::utils;
use super::CmIdx;
use crate::{Cm, PourTxCandidate, TxCandidate};
use crate::{Tx, TxCtrOp, TxType, WASM_MAGIC_NUMBER};
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

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

    pub fn downgrade(self) -> MintTxCandidate {
        self.tx_candidate
    }

    pub fn get_cm_pairs(&self) -> Vec<(CmIdx, Cm)> {
        let cms = self.tx_candidate.get_cms();

        vec![(self.cm_idx_1, cms[0])]
    }
}

impl std::fmt::Display for MintTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MintTx[cm_idx_1: {}, tx_candidate:{}]",
            self.cm_idx_1, self.tx_candidate,
        )
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
    pub cm_1: U8Arr32,

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
        cm_1: U8Arr32,
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
            cm_1,
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
        vec![self.cm_1]
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

impl std::fmt::Display for MintTxCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = if self.data.len() > 12 {
            &self.data[..12]
        } else {
            &self.data[..]
        };

        write!(
            f,
            "MintTx[created_at: {}, data: {:?}, author_sig: {}, ctr_addr: {},\
            cm: {:?}, v: {:?}, k: {:?}, s: {:?}]",
            self.created_at,
            data,
            self.author_sig,
            self.ctr_addr,
            self.cm_1,
            self.v,
            self.k,
            self.s,
        )
    }
}
