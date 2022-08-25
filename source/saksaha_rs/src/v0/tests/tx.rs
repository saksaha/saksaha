use sak_crypto::{rand, ScalarExt};
use sak_crypto::{Hasher, Scalar};
use sak_proofs::{CoinProof, MerkleTree, NewCoin, OldCoin, CM_TREE_DEPTH};
use std::collections::HashMap;
use type_extension::U8Array;

#[tokio::test(flavor = "multi_thread")]
pub async fn test_sdk_get_tx() {
    sak_test_utils::init_test_log();

    let conn_node_port: u16 = 34418;

    // hash is hard-coded
    let hash = String::from(
        "21f25438129d314242f3b919d0beb3ab0c219c765d260467eac3e91bf1031683",
    );
    let resp = crate::get_tx(conn_node_port, hash)
        .await
        .unwrap()
        .result
        .unwrap();
    println!("resp : {:?}", resp);
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sak_sdk_get_ch_list() {
//     let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

//     let req = {
//         // let mut args = HashMap::with_capacity(2);
//         // args.insert(String::from("dst_pk"), "her_pk".to_string());

//         let req_type = String::from("get_ch_list");

//         CtrRequest {
//             req_type,
//             args,
//             ctr_call_type: CtrCallType::Query,
//         }
//     };

//     let json_response = query_ctr(ctr_addr, req).await.unwrap();
//     let Query_res = json_response.result.unwrap();
//     let ch_list: Vec<String> = serde_json::from_str(&Query_res.result).unwrap();

//     println!("Channel list : {:?}", ch_list);
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sak_sdk_get_msgs() {
//     let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

//     let req = {
//         let mut args = HashMap::with_capacity(2);
//         args.insert(String::from(ARG_CH_ID), String::from("channel_0"));

//         let req_type = "get_msgs".to_string();

//         CtrRequest {
//             req_type,
//             args,
//             ctr_call_type: CtrCallType::Query,
//         }
//     };

//     let json_response = query_ctr(ctr_addr, req).await.unwrap();
//     let query_res = json_response.result.unwrap();
//     println!(" query_res : {:?}", query_res);
//     let get_msgs: String = serde_json::from_str(&query_res.result).unwrap();

//     println!("get_msgs : {:?}", get_msgs);
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sak_sdk_send_tx_mint() {
//     let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
//     let req_type = String::from("open_channel");

//     let mut arg = HashMap::with_capacity(2);
//     let open_ch_input = {
//         let open_ch_input: Vec<String> = vec![
//             "eph_pk_str".to_string(),
//             "ch_id8".to_string(),
//             "a_pk_sig_encrypted".to_string(),
//             "open_ch_empty".to_string(),
//         ];

//         serde_json::to_string(&open_ch_input).unwrap()
//     };
//     arg.insert(String::from("dst_pk"), "her_pk".to_string());
//     arg.insert(String::from("serialized_input"), open_ch_input);

//     let json_response = send_tx_mint(
//         // Some(ctr_addr),
//         None,
//         req_type,
//         arg,
//         U8Array::new_empty_32(), // cm
//         U8Array::new_empty_32(), // v
//         U8Array::new_empty_32(), // k
//         U8Array::new_empty_32(), // s
//     )
//     .await
//     .unwrap();
//     let result = json_response.result.unwrap();

//     assert_eq!("success", result);
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_sak_sdk_send_tx_pour_for_open_channel() {
//     let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
//     let req_type = String::from("open_channel");

//     let mut arg = HashMap::with_capacity(2);
//     let open_ch_input = {
//         let open_ch_input: Vec<String> = vec![
//             "eph_pk_str".to_string(),
//             "ch_id7".to_string(),
//             "a_pk_sig_encrypted".to_string(),
//             "open_ch_empty".to_string(),
//         ];

//         serde_json::to_string(&open_ch_input).unwrap()
//     };
//     arg.insert(String::from("dst_pk"), "her_pk".to_string());
//     arg.insert(String::from("serialized_input"), open_ch_input);

//     let json_response = send_tx_pour(ctr_addr, req_type, arg).await.unwrap();
//     let result = json_response.result.unwrap();

//     assert_eq!("success", result);
// }

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
