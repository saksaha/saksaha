use crate::{v0::tests::utils::SakCryptoTestUtils, MerkleTreeSim, ScalarExt};

use type_extension::U8Array;

#[test]
fn test_merkle_simulator() {
    SakCryptoTestUtils::init_test();

    let tree_depth = 32;
    let leaf_len = 2;

    let scalar_zero = ScalarExt::parse_arr(&U8Array::new_empty_32()).unwrap();

    let mut cm_vec = vec![];
    for _i in 0..leaf_len {
        cm_vec.push(scalar_zero);
    }

    let cm_vec_len = cm_vec.len() as u32;

    let tree = MerkleTreeSim::init(tree_depth, cm_vec).unwrap();
    let leaf_count = tree.get_leaf_count();
    let merkle_rt = tree.get_merkle_rt();

    println!("leaf_count: {}", leaf_count);
    println!("merkle_rt: {}", merkle_rt);

    assert_eq!(leaf_count, cm_vec_len);
}
