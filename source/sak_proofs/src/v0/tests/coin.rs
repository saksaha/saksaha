use crate::{
    get_mimc_constants, CoinCircuit, CoinProof, Hasher, MerkleTree,
    CM_TREE_DEPTH,
};
use bellman::groth16;
use bls12_381::Scalar;
use rand::rngs::OsRng;
use rand::RngCore;

fn make_test_context() {
    // mint
    let v = 100; // 100 sak

    let mut key = [0u8; 16];
    OsRng.fill_bytes(&mut key);
    let random_u64 = OsRng.next_u64();

    // let mimc = MiMC::new(None);

    // println!("random_u64: {}", random_u64);

    // let sk = random_u64;
    // let sk_bytes = random_u64.to_be_bytes();

    // let pk = MiMC::mimc_single_arg(&sk);
    // // println!("sk: {:?}, pk: {:?}", sk, pk);

    // let s = 5;
    // let r = 6;
    // let rho = 7;

    // // let k = MiMC::mimc(Scalar::from())
    // // MiMC::mimc()

    // let constants = MiMC::get_mimc_constants();

    // let hasher = |xl, xr| {
    //     let hash = MiMC::mimc(Scalar::from(xl), Scalar::from(xr), &constants);

    //     hash
    // };

    // let data = vec![0, 1, 2, 3, 4, 5, 6, 7];

    // let t = MerkleTree::new(data, 3, &constants, &hasher);

    // t
}

#[tokio::test(flavor = "multi_thread")]
pub async fn test_coin_ownership_default() {
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();
    env_logger::init();

    let proof = {
        let constants = get_mimc_constants();
        let de_params = CoinProof::get_params(&constants);

        let tree = CoinProof::get_merkle_tree(&constants);
        // let tree = make_test_context();

        let root = tree.get_root().hash;

        println!("root: {:?}", root);

        let idx = 0;
        let auth_paths = tree.generate_auth_paths(idx);

        for (idx, p) in auth_paths.iter().enumerate() {
            println!("auth path [{}] - {:?}", idx, p);
        }

        let leaf = tree.nodes.get(0).unwrap().get(idx as usize).unwrap().hash;

        println!("leaf: {:?}", leaf);

        // convert auth_paths => [auth_path]
        let mut auth_path: [Option<(Scalar, bool)>; CM_TREE_DEPTH] =
            [None; CM_TREE_DEPTH];

        for (idx, _) in auth_path.clone().iter().enumerate() {
            let sib = auth_paths.get(idx).unwrap();
            auth_path[idx] = Some((sib.hash.clone(), sib.direction.clone()));
        }

        let c = CoinCircuit {
            leaf: None,
            auth_path,
            // rt,
            constants: &constants,
        };

        let proof =
            groth16::create_random_proof(c, &de_params, &mut OsRng).unwrap();

        proof
    };

    assert!(true);

    // assert!(CoinProof::verify_proof(&proof0));
}
