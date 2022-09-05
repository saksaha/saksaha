use super::utils;
use super::CmIdx;
use crate::{Cm, Sn, Tx, TxCtrOp, TxType};
use sak_crypto::sha3::digest::typenum::U8;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PourTx {
    pub tx_candidate: PourTxCandidate,
    pub cm_idxes: Vec<CmIdx>,
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

    pub fn get_sn(&self) -> Vec<Sn> {
        self.tx_candidate.get_sns().to_owned()
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
    pub sns: Vec<Sn>,

    //
    pub sn_count: u128,

    pub cms: Vec<Cm>,

    //
    pub cm_count: u128,

    //
    pub merkle_rt: [u8; 32],

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
        sns: Vec<Sn>,
        cms: Vec<Cm>,
        merkle_rt: [u8; 32],
    ) -> PourTxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));
        let sn_count = sns.len() as u128;
        let cm_count = cms.len() as u128;

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
            sns,
            sn_count,
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

    pub fn get_sns(&self) -> &Vec<Sn> {
        &self.sns
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
            cms: {:?}, cm_count: {}, sns: {:?}, merkle_rt: {:?}]",
            self.created_at,
            data,
            self.author_sig,
            self.ctr_addr,
            self.cms,
            self.cm_count,
            self.sns,
            self.merkle_rt,
        )
    }
}
