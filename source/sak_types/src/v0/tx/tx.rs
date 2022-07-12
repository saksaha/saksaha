use crate::{MintTxCandidate, PourTxCandidate, TypesError};
use serde::{Deserialize, Serialize};

pub const WASM_MAGIC_NUMBER: [u8; 4] = [0x00, 0x61, 0x73, 0x6d];

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Tx {
    Mint(MintTx),
    Pour(PourTx),
}

impl Tx {
    pub fn get_tx_hash(&self) -> &String {
        match self {
            Tx::Mint(t) => t.tx_candidate.get_tx_hash(),
            Tx::Pour(t) => t.tx_candidate.get_tx_hash(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MintTx {
    //
    tx_candidate: MintTxCandidate,

    //
    tx_height: u128,
}

impl MintTx {
    pub fn new(tx_candidate: MintTxCandidate, tx_height: u128) -> MintTx {
        MintTx {
            tx_candidate,
            tx_height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PourTx {
    //
    tx_candidate: PourTxCandidate,

    //
    tx_height: u128,
}

impl PourTx {
    pub fn new(tx_candidate: PourTxCandidate, tx_height: u128) -> PourTx {
        PourTx {
            tx_candidate,
            tx_height,
        }
    }
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

pub mod for_testing {
    use super::*;

    impl Tx {
        pub fn new_dummy_pour_tx_1() -> Result<Tx, TypesError> {
            let c = PourTxCandidate::new_dummy_1();
            Ok(c.upgrade(0))
        }

        pub fn new_dummy_pour_tx_2() -> Result<Tx, TypesError> {
            let c = PourTxCandidate::new_dummy_2();

            Ok(c.upgrade(1))
        }

        pub fn new_dummy_pour_tx_3() -> Result<Tx, TypesError> {
            let c = PourTxCandidate::new_dummy_3();

            Ok(c.upgrade(2))
        }

        pub fn new_dummy_pour_tx_4() -> Result<Tx, TypesError> {
            let c = PourTxCandidate::new_dummy_4();

            Ok(c.upgrade(3))
        }
    }
}
