use crate::{log, paths::Paths, CIError};
use sak_proofs::{
    groth16, Bls12, CoinProofCircuit1to2, Hasher, NewCoin, OldCoin,
};
use std::fs::File;

const PARAM_FILE_NAME: &str = "mimc_params_1_to_2";

pub(crate) fn build_circuit_params() -> Result<(), CIError> {
    log!("Build circuit params");

    let hasher = Hasher::new();

    // let constants = hasher.get_mimc_constants().to_vec();

    // let coin_1_old = OldCoin::default();
    // let coin_1_new = NewCoin::default();
    // let coin_2_new = NewCoin::default();

    // let params = {
    //     let c = CoinProofCircuit1to2 {
    //         hasher,
    //         coin_1_old,
    //         coin_1_new,
    //         coin_2_new,
    //         constants: constants.to_vec(),
    //     };

    //     groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
    //         .unwrap()
    // };
    // // write param to file
    // let mut file = File::create(PARAM_FILE_NAME)?;

    // params.write(&mut v)?;

    // // write origin buf
    // match file.write_all(&v) {
    //     Ok(_) => {}
    //     Err(err) => {
    //         log::error!("Err: {:?}", err);
    //     }
    // };

    Ok(())
}
