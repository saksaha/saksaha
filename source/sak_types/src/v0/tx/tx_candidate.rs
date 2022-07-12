use crate::{MintTx, PourTx, Tx, TxCtrOp, TypesError, WASM_MAGIC_NUMBER};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq)]
pub enum TxCandidate {
    Mint(MintTxCandidate),
    Pour(PourTxCandidate),
}

impl TxCandidate {
    pub(crate) fn upgrade(self, tx_height: u128) -> Tx {
        match self {
            TxCandidate::Mint(c) => c.upgrade(tx_height),
            TxCandidate::Pour(c) => c.upgrade(tx_height),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct MintTxCandidate {
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    //
    author_sig: String,

    //
    ctr_addr: Option<String>,

    cm: Vec<u8>,
    v: String,
    k: String,
    s: String,

    tx_hash: String,
}

impl MintTxCandidate {
    pub(crate) fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cm: Vec<u8>,
        v: String,
        k: String,
        s: String,
    ) -> MintTxCandidate {
        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
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

    pub(crate) fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }

    pub(crate) fn upgrade(self, tx_height: u128) -> Tx {
        Tx::Mint(MintTx::new(self, tx_height))
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PourTxCandidate {
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    //
    author_sig: String,

    //
    ctr_addr: Option<String>,

    pi: Vec<u8>,
    sn_1: Vec<u8>,
    sn_2: Vec<u8>,
    cm_1: Vec<u8>,
    cm_2: Vec<u8>,
    merkle_rt: Vec<u8>,

    tx_hash: String,
}

impl PourTxCandidate {
    pub(crate) fn new(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        pi: Vec<u8>,
        sn_1: Vec<u8>,
        sn_2: Vec<u8>,
        cm_1: Vec<u8>,
        cm_2: Vec<u8>,
        merkle_rt: Vec<u8>,
    ) -> PourTxCandidate {
        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
        ];

        let tx_hash = sak_crypto::compute_hash(&hashable_items);

        PourTxCandidate {
            created_at,
            data,
            author_sig,
            ctr_addr,
            pi,
            sn_1,
            sn_2,
            cm_1,
            cm_2,
            merkle_rt,
            tx_hash,
        }
    }

    pub(crate) fn upgrade(self, tx_height: u128) -> Tx {
        Tx::Pour(PourTx::new(self, tx_height))
    }

    pub(crate) fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }
}

pub mod for_testing {
    use super::*;

    impl PourTxCandidate {
        pub fn new_dummy_1() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_1"),
                vec![11, 11, 11],
                String::from("author_sig_1"),
                Some(String::from("ctr_addr_1")),
                vec![11, 11, 11],
                vec![11, 11, 11],
                vec![11, 11, 11],
                vec![11, 11, 11],
                vec![11, 11, 11],
                vec![11, 11, 11],
            )
        }

        pub fn new_dummy_2() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_2"),
                vec![22, 22, 22],
                String::from("author_sig_2"),
                Some(String::from("ctr_addr_2")),
                vec![22, 22, 22],
                vec![22, 22, 22],
                vec![22, 22, 22],
                vec![22, 22, 22],
                vec![22, 22, 22],
                vec![22, 22, 22],
            )
        }

        pub fn new_dummy_3() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_3"),
                vec![33, 33, 33],
                String::from("author_sig_3"),
                Some(String::from("ctr_addr_3")),
                vec![33, 33, 33],
                vec![33, 33, 33],
                vec![33, 33, 33],
                vec![33, 33, 33],
                vec![33, 33, 33],
                vec![33, 33, 33],
            )
        }

        pub fn new_dummy_4() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_4"),
                vec![44, 44, 44],
                String::from("author_sig_4"),
                Some(String::from("ctr_addr_4")),
                vec![44, 44, 44],
                vec![44, 44, 44],
                vec![44, 44, 44],
                vec![44, 44, 44],
                vec![44, 44, 44],
                vec![44, 44, 44],
            )
        }
    }
}
