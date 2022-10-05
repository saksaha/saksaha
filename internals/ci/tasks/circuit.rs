use crate::{log, paths::Paths, CIError};
use chrono::Local;
use colored::Colorize;
use sak_crypto::{hasher::MiMC, OsRng};
use sak_proof_circuit::{
    groth16, Bls12, CoinProofCircuit1to2, CoinProofCircuit2to2, NewCoin, OldCoin,
};

pub(crate) fn build_circuit_params() -> Result<(), CIError> {
    let start_time = Local::now();

    log!("Build circuit params 1 to 2, this may take seconds to even minutes..!");
    log!(
        "Build circuit params, start time: {}",
        start_time.format("%H:%M:%S").to_string().yellow(),
    );

    build_circuit_params_1_to_2()?;

    let end_time = Local::now();

    log!(
        "Success generating params 1 to 2, end time: {}",
        end_time.format("%H:%M:%S").to_string().yellow(),
    );

    log!("Build circuit params 2 to 2, this may take seconds to even minutes..!");
    log!(
        "Build circuit params, start time: {}",
        end_time.format("%H:%M:%S").to_string().yellow(),
    );

    build_circuit_params_2_to_2()?;

    let end_time = Local::now();

    log!(
        "Success generating params 2 to 2, end time: {}",
        end_time.format("%H:%M:%S").to_string().yellow(),
    );

    Ok(())
}

fn build_circuit_params_1_to_2() -> Result<(), CIError> {
    let hasher = MiMC::new();
    let constants = hasher.get_mimc_constants().to_vec();
    let coin_1_old = OldCoin::default();
    let coin_1_new = NewCoin::default();
    let coin_2_new = NewCoin::default();

    let params = {
        let c = CoinProofCircuit1to2 {
            hasher,
            coin_1_old,
            coin_1_new,
            coin_2_new,
            constants: constants.to_vec(),
        };

        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
            .expect("Params for Circuit1to2 should be generated")
    };

    let mut v = vec![];
    params.write(&mut v)?;

    let path = Paths::prebuild()?;

    let file_path = path.join("circuit_params_1to2");

    log!("Writing generated circuit params at {:?}", file_path);

    std::fs::write(file_path, v)?;

    Ok(())
}

fn build_circuit_params_2_to_2() -> Result<(), CIError> {
    let hasher = MiMC::new();
    let constants = hasher.get_mimc_constants().to_vec();
    let coin_1_old = OldCoin::default();
    let coin_2_old = OldCoin::default();
    let coin_1_new = NewCoin::default();
    let coin_2_new = NewCoin::default();

    let params = {
        let c = CoinProofCircuit2to2 {
            hasher,
            coin_1_old,
            coin_2_old,
            coin_1_new,
            coin_2_new,
            constants: constants.to_vec(),
        };

        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut OsRng)
            .expect("Params for Circuit2to2 should be generated")
    };

    let mut v = vec![];
    params.write(&mut v)?;

    let path = Paths::prebuild()?;

    let file_path = path.join("circuit_params_2to2");

    log!("Writing generated circuit params at {:?}", file_path);

    std::fs::write(file_path, v)?;

    Ok(())
}
