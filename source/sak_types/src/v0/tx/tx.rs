use crate::{MintTxCandidate, PourTxCandidate, TxCandidate, TypesError};
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
}

pub mod for_testing {
    use super::*;

    impl Tx {
        pub fn new_dummy_tx_1() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_pour_tx_candidate_1();
            Ok(c.upgrade(0))
        }

        pub fn new_dummy_tx_2() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_pour_tx_candidate_2();

            Ok(c.upgrade(1))
        }

        pub fn new_dummy_tx_3() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_pour_tx_candidate_3();

            Ok(c.upgrade(2))
        }

        pub fn new_dummy_tx_4() -> Result<Tx, TypesError> {
            let c = TxCandidate::new_dummy_pour_tx_candidate_4();

            Ok(c.upgrade(3))
        }
    }
}
