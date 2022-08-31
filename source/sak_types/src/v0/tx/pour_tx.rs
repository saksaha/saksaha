use super::utils;
use super::CmIdx;
use crate::{Cm, Sn, Tx, TxCtrOp, TxType};
use serde::{Deserialize, Serialize};
use type_extension::U8Arr32;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PourTx {
    //
    pub tx_candidate: PourTxCandidate,
    pub cm_idxes: Vec<CmIdx>,
    // pub cm_idx_1: CmIdx,
    // pub cm_idx_2: CmIdx,
    // pub tx_height: u128,
}

impl PourTx {
    pub fn new(tx_candidate: PourTxCandidate, cm_idxes: Vec<CmIdx>) -> PourTx {
        PourTx {
            tx_candidate,
            cm_idxes,
        }
    }

    pub fn downgrade(self) -> PourTxCandidate {
        self.tx_candidate
    }

    pub fn get_cm_pairs(&self) -> Vec<(CmIdx, Cm)> {
        let cms = self.tx_candidate.get_cms();

        self.cm_idxes
            .iter()
            .cloned()
            .zip(cms.iter().cloned())
            .collect::<Vec<(CmIdx, Cm)>>()
    }

    pub fn get_sn(&self) -> Sn {
        self.tx_candidate.sn_1
    }
}

impl std::fmt::Display for PourTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PourTx [cm_idxes: {:?}, tx_candidate: {}]",
            self.cm_idxes, self.tx_candidate,
        )
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
    // pub cm_1: U8Arr32,

    // //
    // pub cm_2: U8Arr32,
    pub cms: Vec<Cm>,

    //
    pub cm_count: u128,

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
        cms: Vec<Cm>,
        cm_count: u128,
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
            cms,
            cm_count,
            merkle_rt,
            tx_hash,
        }
    }

    pub fn upgrade(self, cm_idx: CmIdx) -> Tx {
        Tx::Pour(PourTx::new(self, vec![cm_idx, cm_idx + 1]))
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

    pub fn get_cms(&self) -> &Vec<Cm> {
        &self.cms
    }
}

impl std::fmt::Display for PourTxCandidate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data = if self.data.len() > 12 {
            &self.data[..12]
        } else {
            &self.data[..]
        };

        write!(
            f,
            "PourTx [created_at: {}, data: {:?}, author_sig: {}, ctr_addr: {},\
            cms: {:?}, cm_count: {}, sn_1: {:?}, merkle_rt: {:?}]",
            self.created_at,
            data,
            self.author_sig,
            self.ctr_addr,
            self.cms,
            self.cm_count,
            self.sn_1,
            self.merkle_rt,
        )
    }
}
