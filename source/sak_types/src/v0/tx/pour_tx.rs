use super::utils;
use super::CmIdx;
use crate::{Cm, TxCandidate};
use crate::{Tx, TxCtrOp, TxType, WASM_MAGIC_NUMBER};
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

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

    pub fn downgrade(self) -> PourTxCandidate {
        self.tx_candidate
    }

    pub fn get_cm_pairs(&self) -> Vec<(CmIdx, Cm)> {
        let cms = self.tx_candidate.get_cms();

        vec![(self.cm_idx_1, cms[0]), (self.cm_idx_2, cms[1])]
    }
}

impl std::fmt::Display for PourTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // match self {
        //     Tx::Pour(t) => write!(f, "{}", t),
        //     Tx::Mint(t) => write!(f, "{}", t),
        // }
        write!(f, "")
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
