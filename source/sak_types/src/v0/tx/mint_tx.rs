use super::utils;
use super::CmIdx;
use crate::Cm;
use crate::{Tx, TxCtrOp, TxType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MintTx {
    //
    pub tx_candidate: MintTxCandidate,

    //
    pub cm_idxes: Vec<CmIdx>,
}

impl MintTx {
    pub fn new(tx_candidate: MintTxCandidate, cm_idxes: Vec<CmIdx>) -> MintTx {
        MintTx {
            tx_candidate,
            cm_idxes,
        }
    }

    pub fn downgrade(self) -> MintTxCandidate {
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

    pub fn get_cm_idxes(&self) -> &Vec<CmIdx> {
        &self.cm_idxes
    }
}

impl std::fmt::Display for MintTx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MintTx[cm_idxes: {:?}, tx_candidate:{}]",
            self.cm_idxes, self.tx_candidate,
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
    pub cms: Vec<Cm>,

    //
    pub cm_count: u128,

    //
    pub v: [u8; 32],

    //
    pub k: [u8; 32],

    //
    pub s: [u8; 32],

    //
    tx_hash: String,
}

impl MintTxCandidate {
    pub fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cms: Vec<Cm>,
        v: [u8; 32],
        k: [u8; 32],
        s: [u8; 32],
    ) -> MintTxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));
        let cm_count = cms.len() as u128;

        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_bytes(),
            &v,
            &k,
            &s,
        ];

        let tx_hash = sak_crypto::compute_hash(&hashable_items);

        MintTxCandidate {
            created_at,
            data,
            author_sig,
            ctr_addr,
            cms,
            cm_count,
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

    pub fn get_cms(&self) -> &Vec<Cm> {
        &self.cms
    }

    pub fn upgrade(
        self,
        // tx_height: u128,
        cm_idx: CmIdx,
    ) -> Tx {
        Tx::Mint(MintTx::new(self, vec![cm_idx]))
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
            cms: {:?}, cm_count: {}, v: {:?}, k: {:?}, s: {:?}]",
            self.created_at,
            data,
            self.author_sig,
            self.ctr_addr,
            self.cms,
            self.cm_count,
            self.v,
            self.k,
            self.s,
        )
    }
}
