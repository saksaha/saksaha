use crate::hasher::MiMC;
use crate::{v0::tests::utils::SakCryptoTestUtils, MerkleTreeSim, ScalarExt};
use std::collections::HashMap;
use type_extension::U8Array;

#[test]
fn test_merkle_simulator() {
    SakCryptoTestUtils::init_test();
    let hasher = MiMC::new();

    let tree_depth = 6;

    let scalar_zero = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();

    let cm_vec = vec![
        scalar_zero,
        scalar_zero,
        scalar_zero,
        scalar_zero,
        scalar_zero,
    ];
    let cm_vec_len = cm_vec.len() as u32;

    let tree = MerkleTreeSim::init(tree_depth, cm_vec);
    let leaf_count = tree.get_leaf_count();
    let merkle_rt = tree.get_merkle_rt();

    let mut merkle_node = hasher
        .mimc(&U8Array::new_empty_32(), &U8Array::new_empty_32())
        .unwrap();

    for _i in 0..tree_depth - 1 {
        merkle_node = hasher.mimc_scalar(
            merkle_node,
            ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap(),
        );
    }

    println!("leaf_count: {}", leaf_count);
    println!("merkle_rt: {}", merkle_rt);

    assert_eq!(leaf_count, cm_vec_len);
    assert_eq!(merkle_rt, merkle_node);
}
