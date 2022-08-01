// use super::fs;
mod utils;
use crate::db::tests::utils::*;
use crate::db::EnvelopeDB;
use crate::db::{USER_1, USER_2};
use saksaha::*;
use std::time::Duration;
use std::{collections::HashMap, thread::sleep};

#[tokio::test(flavor = "multi_thread")]
async fn test_envelope_db_init() {
    let test_string = String::from("test");
    let db = EnvelopeDB::init(&test_string).await.unwrap();

    db.register_user(&USER_1.to_string()).await.unwrap();
    db.register_user(&USER_2.to_string()).await.unwrap();

    let user_1_sk = db
        .schema
        .get_my_sk(&USER_1.to_string())
        .await
        .unwrap()
        .unwrap();
    let my_pk = db.schema.get_my_pk(&user_1_sk).await.unwrap();
    let my_sig = db.schema.get_my_sig(&user_1_sk).await.unwrap();

    println!("user_1_sk : {:?}", user_1_sk);
    println!("my_pk : {:?}", my_pk);
    println!("my_sig : {:?}", my_sig);

    // let (_a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, credential) =
    //     make_envelope_test_context(&user1, &user2);

    // let eph_pk_str = pk_serialize(eph_pk);

    // let b_pk_str = pk_serialize(b_pk);

    // let (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a) = {
    //     let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, b_pk);

    //     let a_credential_encrypted = {
    //         let ciphertext =
    //             sak_crypto::aes_encrypt(&aes_key_from_a, credential.as_bytes())
    //                 .unwrap();
    //         vec_serialize(&ciphertext)
    //     };

    //     let empty_chat: Vec<String> = vec![];
    //     let empty_chat_str = vec_serialize(&empty_chat);
    //     let ciphertext_empty =
    //         sak_crypto::aes_encrypt(&aes_key_from_a, empty_chat_str.as_bytes())
    //             .unwrap();
    //     let open_ch_empty = vec_serialize(&ciphertext_empty);

    //     (a_credential_encrypted, open_ch_empty, aes_key_from_a)
    // };

    // let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
    // let ch_id = DUMMY_CHANNEL_ID_1.to_string();

    // // insert ch key store
    // user1.insert_ch_key(ch_id.clone(), aes_key_from_a);

    // println!(" ********************** open_channel ********************** ");

    // let open_ch_input = {
    //     let open_ch_input: Vec<String> = vec![
    //         eph_pk_str,
    //         DUMMY_CHANNEL_ID_1.to_string(),
    //         a_pk_sig_encrypted,
    //         open_ch_empty,
    //     ];

    //     serde_json::to_string(&open_ch_input).unwrap()
    // };

    // let mut arg = HashMap::with_capacity(10);
    // arg.insert(String::from(ARG_DST_PK), b_pk_str.clone());
    // arg.insert(String::from(ARG_SERIALIZED_INPUT), open_ch_input);

    // let req_type = String::from("open_channel");
    // let json_response =
    //     send_tx_pour(ctr_addr.clone(), req_type, arg).await.unwrap();
    // let result = json_response.result.unwrap();

    // println!(" Open channel result: {:?}", result);
}
