use crate::{MintTxCandidate, PourTxCandidate, TxCandidateVariant, TypesError};
use serde::{Deserialize, Serialize};

pub const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct Tx {
    //
    tx_candidate: TxCandidate,

    //
    tx_height: u128,
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

impl Tx {
    pub fn new(tx_candidate: TxCandidate, tx_height: u128) -> Tx {
        Tx {
            tx_candidate,
            tx_height,
        }
    }

    pub fn get_created_at(&self) -> &String {
        &self.tx_candidate.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.tx_candidate.data
    }

    pub fn get_pi(&self) -> Option<&Vec<u8>> {
        self.tx_candidate.get_pi()
    }

    pub fn get_author_sig(&self) -> &String {
        &self.tx_candidate.author_sig
    }

    pub fn get_ctr_addr(&self) -> &String {
        &self.tx_candidate.ctr_addr
    }

    pub fn get_tx_height(&self) -> &u128 {
        &self.tx_height
    }

    pub fn get_cm(&self) -> Option<&Vec<u8>> {
        self.tx_candidate.get_cm()
    }

    pub fn get_v(&self) -> Option<&String> {
        self.tx_candidate.get_v()
    }

    pub fn get_k(&self) -> Option<&String> {
        self.tx_candidate.get_k()
    }

    pub fn get_s(&self) -> Option<&String> {
        self.tx_candidate.get_s()
    }

    pub fn get_sn_1(&self) -> Option<&String> {
        self.tx_candidate.get_sn_1()
    }

    pub fn get_sn_2(&self) -> Option<&String> {
        self.tx_candidate.get_sn_2()
    }

    pub fn get_cm_1(&self) -> Option<&Vec<u8>> {
        self.tx_candidate.get_cm_1()
    }

    pub fn get_cm_2(&self) -> Option<&Vec<u8>> {
        self.tx_candidate.get_cm_2()
    }

    pub fn get_merkle_rt(&self) -> Option<&Vec<u8>> {
        self.tx_candidate.get_merkle_rt()
    }

    pub fn get_tx_hash(&self) -> &String {
        &self.tx_candidate.tx_hash
    }

    pub fn is_mutating_ctr_state(&self) -> bool {
        self.tx_candidate.ctr_addr.len() > 0
    }

    pub fn has_ctr_addr(&self) -> bool {
        self.tx_candidate.ctr_addr.len() > 0
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        self.tx_candidate.get_ctr_op()
    }

    pub fn get_tx_variant(&self) -> &TxCandidateVariant {
        &self.tx_candidate.get_tx_variant()
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct TxCandidate {
    //
    created_at: String,

    //
    #[serde(with = "serde_bytes")]
    data: Vec<u8>,
    //
    author_sig: String,

    //
    ctr_addr: Option<String>,

    //
    variant: TxCandidateVariant,

    // auto generated value
    tx_hash: String,
}

impl TxCandidate {
    // pub fn new(
    //     created_at: String,
    //     data: Vec<u8>,
    //     author_sig: String,
    //     ctr_addr: Option<String>,
    //     pi: Option<Vec<u8>>,
    //     cm: Option<Vec<u8>>,
    //     v: Option<String>,
    //     k: Option<String>,
    //     s: Option<String>,
    //     sn_1: Option<String>,
    //     sn_2: Option<String>,
    //     cm_1: Option<Vec<u8>>,
    //     cm_2: Option<Vec<u8>>,
    //     merkle_rt: Option<Vec<u8>>,
    // ) -> Result<TxCandidate, TypesError> {
    //     let variant = match (cm, v, k, s, pi, sn_1, sn_2, cm_1, cm_2, merkle_rt)
    //     {
    //         (
    //             Some(cm),
    //             Some(v),
    //             Some(k),
    //             Some(s),
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //             None,
    //         ) => {
    //             let variant =
    //                 TxCandidateVariant::Mint(MintTxCandidate { cm, v, k, s });

    //             variant
    //         }
    //         (
    //             None,
    //             None,
    //             None,
    //             None,
    //             Some(pi),
    //             Some(sn_1),
    //             Some(sn_2),
    //             Some(cm_1),
    //             Some(cm_2),
    //             Some(merkle_rt),
    //         ) => {
    //             let variant = TxCandidateVariant::Pour(PourTxCandidate {
    //                 pi,
    //                 sn_1,
    //                 sn_2,
    //                 cm_1,
    //                 cm_2,
    //                 merkle_rt,
    //             });

    //             variant
    //         }
    //         _ => {
    //             return Err(
    //                 format!("Tx candidate arguments are invalid.").into()
    //             )
    //         }
    //     };

    //     let ctr_addr = ctr_addr.unwrap_or(String::from(""));

    //     let mut hashable_items = variant.get_hashable_items();

    //     let mut extra_hashable_items = vec![
    //         created_at.as_bytes(),
    //         data.as_slice(),
    //         author_sig.as_bytes(),
    //     ];

    //     hashable_items.append(&mut extra_hashable_items);

    //     let tx_hash = sak_crypto::compute_hash(&hashable_items);

    //     let c = TxCandidate {
    //         created_at,
    //         data,
    //         author_sig,
    //         ctr_addr,
    //         variant,
    //         tx_hash,
    //     };

    //     Ok(c)
    // }

    pub fn new_mint_tx_candidate(
        created_at: String,
        data: Vec<u8>,
        author_sig: String,
        ctr_addr: Option<String>,
        cm: Vec<u8>,
        v: String,
        k: String,
        s: String,
    ) -> TxCandidate {
        let variant = TxCandidateVariant::Mint(MintTxCandidate { cm, v, k, s });

        let mut hashable_items = variant.get_hashable_items();

        let mut extra_hashable_items = vec![
            created_at.as_bytes(),
            data.as_slice(),
            author_sig.as_bytes(),
        ];

        hashable_items.append(&mut extra_hashable_items);

        let tx_hash = sak_crypto::compute_hash(&hashable_items);

        TxCandidate {
            variant,
            created_at,
            data,
            author_sig,
            ctr_addr,
            tx_hash,
        }
    }

    pub fn new_pour_tx_candidate() {}

    pub fn upgrade(self, tx_height: u128) -> Tx {
        Tx::new(self, tx_height)
    }

    pub fn get_created_at(&self) -> &String {
        &self.created_at
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_pi(&self) -> Option<&Vec<u8>> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.pi);
        } else {
            return None;
        }
    }

    pub fn get_author_sig(&self) -> &String {
        &self.author_sig
    }

    pub fn get_ctr_addr(&self) -> &String {
        &self.ctr_addr
    }

    pub fn get_cm(&self) -> Option<&Vec<u8>> {
        if let TxCandidateVariant::Mint(v) = &self.variant {
            return Some(&v.cm);
        } else {
            return None;
        }
    }

    pub fn get_v(&self) -> Option<&String> {
        if let TxCandidateVariant::Mint(v) = &self.variant {
            return Some(&v.v);
        } else {
            return None;
        }
    }

    pub fn get_k(&self) -> Option<&String> {
        if let TxCandidateVariant::Mint(v) = &self.variant {
            return Some(&v.k);
        } else {
            return None;
        }
    }

    pub fn get_s(&self) -> Option<&String> {
        if let TxCandidateVariant::Mint(v) = &self.variant {
            return Some(&v.s);
        } else {
            return None;
        }
    }

    pub fn get_sn_1(&self) -> Option<&String> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.sn_1);
        } else {
            return None;
        }
    }

    pub fn get_sn_2(&self) -> Option<&String> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.sn_2);
        } else {
            return None;
        }
    }

    pub fn get_cm_1(&self) -> Option<&Vec<u8>> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.cm_1);
        } else {
            return None;
        }
    }

    pub fn get_cm_2(&self) -> Option<&Vec<u8>> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.cm_2);
        } else {
            return None;
        }
    }

    pub fn get_merkle_rt(&self) -> Option<&Vec<u8>> {
        if let TxCandidateVariant::Pour(v) = &self.variant {
            return Some(&v.merkle_rt);
        } else {
            return None;
        }
    }

    pub fn get_tx_hash(&self) -> &String {
        &self.tx_hash
    }

    pub fn get_tx_variant(&self) -> &TxCandidateVariant {
        &self.variant
    }

    pub fn is_mutating_ctr_state(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn has_ctr_addr(&self) -> bool {
        self.ctr_addr.len() > 0
    }

    pub fn get_ctr_op(&self) -> TxCtrOp {
        get_ctr_op(&self.ctr_addr, &self.data)
    }
}

fn get_ctr_op(ctr_addr: &String, data: &Vec<u8>) -> TxCtrOp {
    let tx_ctr_type = {
        let mut c = TxCtrOp::None;
        if ctr_addr.len() > 0 {
            if data.len() > 4 {
                if data[0..4] == WASM_MAGIC_NUMBER {
                    c = TxCtrOp::ContractDeploy;
                } else {
                    c = TxCtrOp::ContractCall;
                }
            }
        }
        c
    };

    return tx_ctr_type;
}

pub mod for_testing {
    use super::*;

    impl TxCandidate {
        pub fn new_dummy_tx_candidate_1() -> Result<TxCandidate, TypesError> {
            TxCandidate::new(
                String::from("1"),
                vec![11, 11, 11],
                String::from("1"),
                Some(b"1".to_vec()),
                Some(String::from("11")),
                Some(vec![11, 11, 11]),
                Some(String::from("11")),
                Some(String::from("11")),
                Some(String::from("11")),
                Some(String::from("11")),
                Some(String::from("11")),
                Some(vec![11, 11, 11]),
                Some(vec![11, 11, 11]),
                Some(vec![11, 11, 11]),
            )
        }

        pub fn new_dummy_tx_candidate_2() -> Result<TxCandidate, TypesError> {
            TxCandidate::new(
                String::from("2"),
                vec![2, 2, 2],
                String::from("2"),
                Some(b"2".to_vec()),
                Some(String::from("22")),
                Some(vec![2, 2, 2]),
                Some(String::from("22")),
                Some(String::from("22")),
                Some(String::from("22")),
                Some(String::from("22")),
                Some(String::from("22")),
                Some(vec![2, 2, 2]),
                Some(vec![2, 2, 2]),
                Some(vec![2, 2, 2]),
            )
        }

        pub fn new_dummy_tx_candidate_3() -> Result<TxCandidate, TypesError> {
            TxCandidate::new(
                String::from("3"),
                vec![3, 3, 3],
                String::from("3"),
                Some(b"3".to_vec()),
                Some(String::from("33")),
                Some(vec![33, 33, 33]),
                Some(String::from("33")),
                Some(String::from("33")),
                Some(String::from("33")),
                Some(String::from("33")),
                Some(String::from("33")),
                Some(vec![33, 33, 33]),
                Some(vec![33, 33, 33]),
                Some(vec![33, 33, 33]),
            )
        }

        pub fn new_dummy_tx_candidate_4() -> Result<TxCandidate, TypesError> {
            TxCandidate::new(
                String::from("4"),
                vec![4, 4, 4],
                String::from("4"),
                Some(b"4".to_vec()),
                Some(String::from("44")),
                Some(vec![44, 44, 44]),
                Some(String::from("44")),
                Some(String::from("44")),
                Some(String::from("44")),
                Some(String::from("44")),
                Some(String::from("44")),
                Some(vec![44, 44, 44]),
                Some(vec![44, 44, 44]),
                Some(vec![44, 44, 44]),
            )
        }
    }

    impl Tx {
        pub fn new_dummy_tx_1() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_tx_candidate_1()?;
            Ok(c.upgrade(0))
        }

        pub fn new_dummy_tx_2() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_tx_candidate_2()?;

            Ok(c.upgrade(1))
        }

        pub fn new_dummy_tx_3() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_tx_candidate_3()?;

            Ok(c.upgrade(2))
        }

        pub fn new_dummy_tx_4() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_tx_candidate_4()?;

            Ok(c.upgrade(3))
        }
    }
}
