use crate::{v0::tests::utils::SakCryptoTestUtils, MerkleTreeSim};

#[test]
fn test_merkle_simulator() {
    SakCryptoTestUtils::init_test();

    let tree = MerkleTreeSim::new(5);

    println!("11");
}
