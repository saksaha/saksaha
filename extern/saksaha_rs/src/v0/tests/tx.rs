use crate::v0::sdk::*;

use super::utils::*;
use sak_crypto::{
    PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};
use std::collections::HashMap;

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_get_ch_list() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    arg.insert(String::from("dst_pk"), "her_pk".to_string());

    let req_type = String::from("get_ch_list");

    let json_response = call_contract(ctr_addr, req_type, arg).await.unwrap();
    let Query_res = json_response.result.unwrap();
    let ch_list: Vec<String> = serde_json::from_str(&Query_res.result).unwrap();

    println!("Channel list : {:?}", ch_list);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_get_msgs() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    arg.insert(String::from(ARG_CH_ID), String::from("channel_0"));

    let req_type = "get_msgs".to_string();

    let json_response = call_contract(ctr_addr, req_type, arg).await.unwrap();
    let query_res = json_response.result.unwrap();
    println!(" query_res : {:?}", query_res);
    let get_msgs: String = serde_json::from_str(&query_res.result).unwrap();

    println!("get_msgs : {:?}", get_msgs);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_send_tx_mint() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    let open_ch_input = {
        let open_ch_input: Vec<String> = vec![
            "eph_pk_str".to_string(),
            "ch_id8".to_string(),
            "a_pk_sig_encrypted".to_string(),
            "open_ch_empty".to_string(),
        ];

        serde_json::to_string(&open_ch_input).unwrap()
    };
    arg.insert(String::from("dst_pk"), "her_pk".to_string());
    arg.insert(String::from("serialized_input"), open_ch_input);

    let req_type = String::from("open_channel");
    let json_response = send_tx_mint(ctr_addr, req_type, arg).await.unwrap();
    let result = json_response.result.unwrap();

    assert_eq!("success", result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_sdk_send_tx_pour_for_open_channel() {
    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let mut arg = HashMap::with_capacity(2);
    let open_ch_input = {
        let open_ch_input: Vec<String> = vec![
            "eph_pk_str".to_string(),
            "ch_id7".to_string(),
            "a_pk_sig_encrypted".to_string(),
            "open_ch_empty".to_string(),
        ];

        serde_json::to_string(&open_ch_input).unwrap()
    };
    arg.insert(String::from("dst_pk"), "her_pk".to_string());
    arg.insert(String::from("serialized_input"), open_ch_input);

    let req_type = String::from("open_channel");
    let json_response = send_tx_pour(ctr_addr, req_type, arg).await.unwrap();
    let result = json_response.result.unwrap();

    assert_eq!("success", result);
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sak_sdk_send_tx_pour_for_send_msg() {
//     let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

//     let msg = String::from("Hello, A");

//     let ch_id = "ch_id7".to_string();

//     let msgs_serialized = storage.get(&ch_id).unwrap();
//     let ciphertext_msgs: Vec<u8> =
//         serde_json::from_str(msgs_serialized.as_str()).unwrap();

//     let plaintext_msgs =
//         sak_crypto::aes_decrypt(&aes_key, ciphertext_msgs.as_slice()).unwrap();

//     let mut old_chat: Vec<String> =
//         serde_json::from_str(&plaintext_msgs).unwrap();

//     let my_pk_str = pk_serialize(pk);
//     let msg_pk = vec![msg, &my_pk_str];
//     let serialized_msg = vec_serialize(&msg_pk);

//     old_chat.push(serialized_msg);
//     let chat_vec_str = vec_serialize(&old_chat);

//     let ciphertext =
//         sak_crypto::aes_encrypt(&aes_key, chat_vec_str.as_bytes()).unwrap();
//     let ciphertext_str = vec_serialize(&ciphertext);

//     let mut arg = HashMap::with_capacity(2);
//     arg.insert(String::from(ARG_CH_ID), ch_id);
//     arg.insert(String::from(ARG_SERIALIZED_INPUT), ciphertext_str);

//     let req_type = String::from("send_msg");
//     let json_response = send_tx_pour(ctr_addr, req_type, arg).await.unwrap();
//     let result = json_response.result.unwrap();

//     assert_eq!("success", result);
// }
