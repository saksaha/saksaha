use super::utils;
use crate::{MintTx, PourTx, Tx, TxCtrOp, TypesError, WASM_MAGIC_NUMBER};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
pub enum TxCandidate {
    Mint(MintTxCandidate),
    Pour(PourTxCandidate),
}

impl TxCandidate {
    pub fn upgrade(self, tx_height: u128) -> Tx {
        match self {
            TxCandidate::Mint(c) => c.upgrade(tx_height),
            TxCandidate::Pour(c) => c.upgrade(tx_height),
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
    pub cm: [u8; 32],

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
        cm: [u8; 32],
        v: [u8; 32],
        k: [u8; 32],
        s: [u8; 32],
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

    pub fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        utils::get_ctr_op(&self.ctr_addr, &self.data)
    }

    pub fn upgrade(self, tx_height: u128) -> Tx {
        Tx::Mint(MintTx::new(self, tx_height))
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
    pub sn_1: [u8; 32],

    //
    pub sn_2: [u8; 32],

    //
    pub cm_1: [u8; 32],

    //
    pub cm_2: [u8; 32],

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
        sn_1: [u8; 32],
        sn_2: [u8; 32],
        cm_1: [u8; 32],
        cm_2: [u8; 32],
        merkle_rt: [u8; 32],
    ) -> PourTxCandidate {
        let ctr_addr = ctr_addr.unwrap_or(String::from(""));

        let hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
            ctr_addr.as_bytes(),
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

    pub fn upgrade(self, tx_height: u128) -> Tx {
        Tx::Pour(PourTx::new(self, tx_height))
    }

    pub fn get_tx_hash(&self) -> &String {
        return &self.tx_hash;
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        utils::get_ctr_op(&self.ctr_addr, &self.data)
    }
}

pub mod for_testing {
    use sak_crypto::{Hasher, Scalar, ScalarExt};

    use super::*;
    use crate::U8Array;

    impl TxCandidate {
        pub fn new_dummy_mint_1() -> TxCandidate {
            let mint_tx_candidate_dummy_1 = MintTxCandidate::new_dummy_1();

            TxCandidate::Mint(mint_tx_candidate_dummy_1)
        }

        pub fn new_dummy_pour_1() -> TxCandidate {
            let pour_tx_candidate_dummy_1 = PourTxCandidate::new_dummy_1();

            TxCandidate::Pour(pour_tx_candidate_dummy_1)
        }

        pub fn new_dummy_pour_2() -> TxCandidate {
            let pour_tx_candidate_dummy_2 = PourTxCandidate::new_dummy_2();

            TxCandidate::Pour(pour_tx_candidate_dummy_2)
        }

        pub fn new_dummy_pour_3() -> TxCandidate {
            let pour_tx_candidate_dummy_3 = PourTxCandidate::new_dummy_3();

            TxCandidate::Pour(pour_tx_candidate_dummy_3)
        }

        pub fn new_dummy_pour_4() -> TxCandidate {
            let pour_tx_candidate_dummy_4 = PourTxCandidate::new_dummy_4();

            TxCandidate::Pour(pour_tx_candidate_dummy_4)
        }
    }

    impl MintTxCandidate {
        pub fn new_dummy_1() -> MintTxCandidate {
            let hasher = Hasher::new();

            let v = Scalar::from(1_000);

            let s = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr).unwrap()
            };

            let r = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr).unwrap()
            };

            let rho = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr).unwrap()
            };

            let a_pk = {
                let arr = U8Array::new_empty_32();

                ScalarExt::parse_arr(&arr).unwrap()
            };

            let k = hasher.comm2(r, a_pk, rho);

            let cm = hasher.comm2(s, v, k);

            MintTxCandidate::new(
                String::from("created_at_mint_1"),
                vec![1],
                String::from("author_sig_mint_1"),
                None,
                cm.to_bytes(),
                v.to_bytes(),
                k.to_bytes(),
                s.to_bytes(),
            )
        }
    }

    impl PourTxCandidate {
        pub fn new_dummy_1() -> PourTxCandidate {
            let hasher = Hasher::new();

            let pi = vec![0];

            let cm_1 = {
                let v = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let s = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let r = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let rho = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let a_pk = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let k = hasher.comm2(r, a_pk, rho);
                let cm = hasher.comm2(s, v, k);
                cm
            };

            let cm_2 = {
                let v = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let s = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let r = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let rho = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let a_pk = {
                    let arr = U8Array::new_empty_32();
                    ScalarExt::parse_arr(&arr).unwrap()
                };

                let k = hasher.comm2(r, a_pk, rho);
                let cm = hasher.comm2(s, v, k);
                cm
            };

            // let ptc = PourTxCandidate::new(
            //     String::from("initial_mint_created_at"),
            //     vec![0],
            //     VALIDATOR_SIG.to_string(),
            //     None,
            //     pi,
            //     sn_1,
            //     sn_2,
            //     cm_1,
            //     cm_1,
            //     merkle_rt,
            // );

            PourTxCandidate::new(
                String::from("created_at_1"),
                vec![11, 11, 11],
                String::from("author_sig_1"),
                Some(String::from("ctr_addr_1")),
                vec![11, 11, 11],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            )
        }

        pub fn new_dummy_2() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_2"),
                vec![22, 22, 22],
                String::from("author_sig_2"),
                Some(String::from("ctr_addr_2")),
                vec![22, 22, 22],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            )
        }

        pub fn new_dummy_3() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_3"),
                vec![33, 33, 33],
                String::from("author_sig_3"),
                Some(String::from("ctr_addr_3")),
                vec![22, 22, 22],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            )
        }

        pub fn new_dummy_4() -> PourTxCandidate {
            PourTxCandidate::new(
                String::from("created_at_4"),
                vec![44, 44, 44],
                String::from("author_sig_4"),
                Some(String::from("ctr_addr_4")),
                vec![44, 44, 44],
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
                U8Array::new_empty_32(),
            )
        }
    }
}
