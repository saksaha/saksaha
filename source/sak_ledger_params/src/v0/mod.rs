use sak_crypto::{MerkleTreeSim, ScalarExt};
use sak_ledger_cfg::CM_TREE_DEPTH;

use crate::LedgerParamsError;

pub const DUMMY_CM: [u8; 32] = [
    235, 141, 62, 187, 86, 12, 228, 147, 136, 201, 197, 117, 154, 229, 95, 155, 20, 131, 251, 178,
    178, 43, 79, 143, 197, 142, 113, 142, 63, 192, 180, 59,
];

pub const DUMMY_SN: [u8; 32] = [
    214, 107, 131, 229, 87, 169, 202, 14, 124, 201, 178, 160, 124, 64, 127, 131, 1, 79, 76, 17,
    161, 60, 250, 110, 102, 175, 33, 193, 105, 88, 32, 70,
];

pub fn mock_rt_1() -> Result<[u8; 32], LedgerParamsError> {
    let dummy_cm = ScalarExt::parse_arr(&DUMMY_CM)?;

    let mk_tree_init = MerkleTreeSim::init(CM_TREE_DEPTH, vec![dummy_cm])?;

    let dummy_merkle_rt: [u8; 32] = mk_tree_init.get_merkle_rt().to_bytes().into();

    Ok(dummy_merkle_rt)
}
