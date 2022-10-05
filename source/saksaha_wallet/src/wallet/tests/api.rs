use crate::{tests::utils::mock_wallet_context, wallet::tests::utils::mock_open_ch_ctr_request};
use sak_crypto::{Bls12, ScalarExt};
use sak_proof::{CoinProof, MiMC, Proof};
use sak_types::CoinRecord;

#[tokio::test(flavor = "multi_thread")]
async fn test_prepare_send_tx_pour_params() {
    //
    let context = mock_wallet_context().await;

    let wallet = context.wallet;

    let mut coin_manager_lock = wallet.get_coin_manager().write().await;

    let coin: &CoinRecord = coin_manager_lock
        .get_next_available_coin()
        .ok_or("No usable coins")
        .unwrap();

    // let cm_idx = wallet.prepare_cm_idx(coin).await?;
    // let auth_path = wallet.prepare_auth_path(cm_idx).await?;
    let auth_path: Vec<([u8; 32], bool)> = vec![
        (
            [
                183, 140, 126, 139, 38, 63, 12, 79, 128, 44, 123, 134, 90, 86, 52, 66, 107, 188,
                120, 39, 129, 98, 243, 225, 235, 181, 185, 137, 218, 223, 139, 32,
            ],
            false,
        ),
        (
            [
                65, 41, 64, 119, 6, 86, 234, 216, 5, 188, 193, 203, 203, 171, 4, 65, 82, 46, 182,
                40, 171, 80, 229, 44, 254, 179, 48, 201, 104, 216, 191, 50,
            ],
            false,
        ),
        (
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            false,
        ),
        (
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0,
            ],
            false,
        ),
    ];

    let merkle_rt = wallet.prepare_merkle_rt(coin, auth_path.clone()).unwrap();

    let old_coin = wallet.convert_to_old_coin(coin, auth_path).unwrap();

    let old_sn_1 = wallet.compute_sn(coin);

    let (new_coin_1, new_coin_2) = wallet.prepare_2_new_coin_records(coin.v).unwrap();

    let pi_ser = wallet
        .prepare_proof_1_to_2(
            old_coin,
            new_coin_1.extract_new_coin(),
            new_coin_2.extract_new_coin(),
        )
        .unwrap();

    // skip send_pour_tx();

    {
        let pi: Proof<Bls12> = Proof::read(&*pi_ser).unwrap();

        let public_inputs = [
            ScalarExt::parse_arr(&merkle_rt).unwrap(),
            ScalarExt::parse_arr(&old_sn_1).unwrap(),
            new_coin_1.cm,
            new_coin_2.cm,
        ];

        let hasher = MiMC::new();

        let res = CoinProof::verify_proof_1_to_2(pi, &public_inputs, &hasher).unwrap();

        assert_eq!(res, true);
    }
}
