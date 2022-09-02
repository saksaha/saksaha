use crate::{routes::v0::SendTxRequest, tests::utils::mock_wallet_context};
use envelope_contract::{request_type, Channel, OpenChParams};
use sak_contract_std::CtrRequest;
use sak_crypto::{SakKey, ToEncodedPoint};
use sak_types::CoinRecord;
use type_extension::U8Arr32;

#[tokio::test(flavor = "multi_thread")]
async fn test_prepare_send_tx_pour_params() {
    //
    let context = mock_wallet_context().await;

    let wallet = context.wallet;

    let evl_ctr_addr = String::from("envelope_contract_addr");

    let (_, eph_pub_key) = SakKey::generate();

    let channel = Channel::new(
        "ch_id".to_string(),
        sak_crypto::encode_hex(&eph_pub_key.to_encoded_point(false).to_bytes()),
        "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
            .to_string(),
        vec![
            "\
                045739d074b8722891c307e8e75c9607\
                e0b55a80778b42ef5f4640d4949dbf39\
                92f6083b729baef9e9545c4e95590616\
                fd382662a09653f2a966ff524989ae8c0f"
                .to_string(),
            "\
                    042c8d005bd935597117181d8ceceaef\
                    6d1162de78c3285689d0c36c6170634c\
                    124f7b9b911553a1f483ec565c199ea2\
                    9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
                .to_string(),
        ],
    )
    .unwrap();

    let open_ch_params = OpenChParams {
        dst_pk: "\
                042c8d005bd935597117181d8ceceaef\
                6d1162de78c3285689d0c36c6170634c\
                124f7b9b911553a1f483ec565c199ea2\
                9ff1cd641f10c9a5f8c7c4d4a026db6f7b"
            .to_string(),
        open_ch: channel,
    };

    let args = serde_json::to_vec(&open_ch_params).unwrap();

    let ctr_request = CtrRequest {
        req_type: request_type::SEND_MSG.to_string(),
        args,
        ctr_call_type: sak_contract_std::CtrCallType::Execute,
    };

    // let res = wallet
    //     .clone()
    //     .send_pour_tx(context.acc_addr, evl_ctr_addr, ctr_request)
    //     .await
    //     .unwrap();

    // skip the rpc call to the `node`
    let mut coin_manager_lock = wallet.get_coin_manager().write().await;

    let coin: &CoinRecord = coin_manager_lock
        .get_next_available_coin()
        .ok_or("No usable coins")
        .unwrap();

    println!("[test] usable coin: {:?}", coin);

    let cm_idx: u128 = 0;

    let auth_path: Vec<(U8Arr32, bool)> = vec![
        (
            [
                183, 140, 126, 139, 38, 63, 12, 79, 128, 44, 123, 134, 90, 86,
                52, 66, 107, 188, 120, 39, 129, 98, 243, 225, 235, 181, 185,
                137, 218, 223, 139, 32,
            ],
            false,
        ),
        (
            [
                65, 41, 64, 119, 6, 86, 234, 216, 5, 188, 193, 203, 203, 171,
                4, 65, 82, 46, 182, 40, 171, 80, 229, 44, 254, 179, 48, 201,
                104, 216, 191, 50,
            ],
            false,
        ),
        (
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            false,
        ),
        (
            [
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            ],
            false,
        ),
    ];

    let merkle_rt = wallet
        .prepare_merkle_rt(coin, auth_path.clone())
        .await
        .unwrap();

    let old_coin = wallet.convert_to_old_coin(coin, auth_path).await.unwrap();

    let old_sn_1 = wallet.compute_sn(coin);

    let (mut new_coin_1, mut new_coin_2) =
        wallet.prepare_2_new_coin_records(coin.v).unwrap();

    let pi = wallet
        .prepare_proof_1_to_2(
            old_coin,
            new_coin_1.extract_new_coin(),
            new_coin_2.extract_new_coin(),
        )
        .unwrap();

    // verify_proof

    // let json_response = saksaha::send_tx_pour(
    //     wallet.saksaha_endpoint.clone(),
    //     old_sn_1,
    //     new_coin_1.cm.to_bytes(),
    //     new_coin_2.cm.to_bytes(),
    //     merkle_rt,
    //     pi,
    //     evl_ctr_addr,
    //     ctr_request,
    // )
    // .await
    // .unwrap();

    // let tx_hash = json_response
    //     .result
    //     .ok_or("Value needs to be returned")
    //     .unwrap();

    // println!("tx_hash: {:?}", tx_hash);
}
