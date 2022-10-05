use crate::hasher::MiMC;
use crate::{v0::tests::utils::SakCryptoTestUtils, MerkleTreeSim, ScalarExt};
use std::collections::HashMap;
use type_extension::U8Array;

#[test]
fn test_merkle_simulator() {
    SakCryptoTestUtils::init_test();
    let hasher = MiMC::new();

    let tree_depth = 2;

    let scalar_zero = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();

    let cm_vec = vec![scalar_zero, scalar_zero];
    let cm_vec_len = cm_vec.len() as u32;

    let tree = MerkleTreeSim::init(tree_depth, cm_vec);
    let leaf_count = tree.get_leaf_count();
    let merkle_rt = tree.get_merkle_rt();

    let merkle_1_0 = hasher
        .mimc(&U8Array::new_empty_32(), &U8Array::new_empty_32())
        .unwrap();
    let merkle_2_0 = hasher.mimc_scalar(
        merkle_1_0,
        ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap(),
    );

    println!("leaf_count: {}", leaf_count);
    println!("merkle_rt: {}", merkle_rt);
    assert_eq!(leaf_count, cm_vec_len);
    assert_eq!(merkle_rt, merkle_2_0);
}
