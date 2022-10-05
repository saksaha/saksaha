use crate::{v0::tests::utils::SakCryptoTestUtils, MerkleTreeSim, ScalarExt};
use std::collections::HashMap;

#[test]
fn test_merkle_simulator() {
    SakCryptoTestUtils::init_test();

    let tree = MerkleTreeSim::init(
        5,
        vec![
            // some element (scalar)
        ],
    );

    // let merkle_nodes_1 = {
    //     let cm_1 = ScalarExt::parse_arr(&old_coin_1.cm).unwrap();
    //     let cm_2 = ScalarExt::parse_arr(&old_coin_2.cm).unwrap();

    //     let mut m = HashMap::new();

    //     let node_0_1 = cm_2;
    //     let node_1_1 = ScalarExt::parse_u64(0).unwrap();
    //     let node_2_1 = ScalarExt::parse_u64(0).unwrap();
    //     let node_3_1 = ScalarExt::parse_u64(0).unwrap();
    //     let node_4_1 = ScalarExt::parse_u64(0).unwrap();
    //     let node_5_1 = ScalarExt::parse_u64(0).unwrap();

    //     m.insert("0_1", node_0_1);
    //     m.insert("1_1", node_1_1);
    //     m.insert("2_1", node_2_1);
    //     m.insert("3_1", node_3_1);
    //     m.insert("4_1", node_4_1);
    //     m.insert("5_1", node_5_1);

    //     let node_1_0 = hasher.mimc_scalar(cm_1, cm_2);
    //     let node_2_0 = hasher.mimc_scalar(node_1_0, node_1_1);
    //     let node_3_0 = hasher.mimc_scalar(node_2_0, node_2_1);
    //     let node_4_0 = hasher.mimc_scalar(node_3_0, node_3_1);
    //     let node_5_0 = hasher.mimc_scalar(node_4_0, node_4_1);
    //     let node_6_0 = hasher.mimc_scalar(node_5_0, node_5_1);

    //     m.insert("1_0", node_1_0);
    //     m.insert("2_0", node_2_0);
    //     m.insert("3_0", node_3_0);
    //     m.insert("4_0", node_4_0);
    //     m.insert("5_0", node_5_0);
    //     m.insert("6_0", node_6_0);

    //     m
    // };

    println!("11");
}
