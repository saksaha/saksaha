mod utils;

use self::utils::*;
use super::fs;
use crate::pconfig::*;
use sak_contract_std::{CtrCallType, Request as CtrRequest};
use saksaha::*;
use std::time::Duration;
use std::{collections::HashMap, thread::sleep};

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_ecies_open_channel() {
    let mut user1 = PConfig::new(&USER_1.to_string()).unwrap();
    let user2 = PConfig::new(&USER_2.to_string()).unwrap();

    let (_a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, credential) =
        make_envelope_test_context(&user1, &user2);

    let eph_pk_str = pk_serialize(eph_pk);

    let b_pk_str = pk_serialize(b_pk);

    let (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a) = {
        let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, b_pk);

        let a_credential_encrypted = {
            let ciphertext =
                sak_crypto::aes_encrypt(&aes_key_from_a, credential.as_bytes())
                    .unwrap();
            vec_serialize(&ciphertext)
        };

        let empty_chat: Vec<String> = vec![];
        let empty_chat_str = vec_serialize(&empty_chat);
        let ciphertext_empty =
            sak_crypto::aes_encrypt(&aes_key_from_a, empty_chat_str.as_bytes())
                .unwrap();
        let open_ch_empty = vec_serialize(&ciphertext_empty);

        (a_credential_encrypted, open_ch_empty, aes_key_from_a)
    };

    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();
    let ch_id = DUMMY_CHANNEL_ID_1.to_string();

    println!("{}", b_pk_str);

    // insert ch key store
    user1.insert_ch_key(ch_id.clone(), aes_key_from_a);

    println!(" ********************** open_channel ********************** ");

    let open_ch_input = {
        let open_ch_input: Vec<String> = vec![
            eph_pk_str,
            DUMMY_CHANNEL_ID_1.to_string(),
            a_pk_sig_encrypted,
            open_ch_empty,
        ];

        serde_json::to_string(&open_ch_input).unwrap()
    };

    let mut arg = HashMap::with_capacity(10);
    arg.insert(String::from(ARG_DST_PK), b_pk_str.clone());
    arg.insert(String::from(ARG_SERIALIZED_INPUT), open_ch_input);

    let req_type = String::from("open_channel");
    let json_response =
        send_tx_pour(ctr_addr.clone(), req_type, arg).await.unwrap();
    let result = json_response.result.unwrap();

    println!(" Open channel result: {:?}", result);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_sak_ecies_get_msgs_and_send_msg() {
    println!(" ********************** get_msgs start ********************** ");

    let user1 = PConfig::load(&USER_1.to_string()).unwrap();

    let aes_key_from_a =
        user1.get_ch_key(&DUMMY_CHANNEL_ID_1.to_string()).unwrap();

    let ctr_addr = ENVELOPE_CTR_ADDR.to_string();

    let req = {
        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from(ARG_CH_ID), DUMMY_CHANNEL_ID_1.to_string());

        let req_type = "get_msgs".to_string();

        CtrRequest {
            req_type,
            arg,
            ctr_call_type: CtrCallType::Query,
        }
    };

    let mut old_chat: Vec<String> = {
        let json_response = call_contract(ctr_addr.clone(), req).await.unwrap();
        let query_res = json_response.result.unwrap().result;

        let messages: Vec<u8> =
            serde_json::from_str(query_res.as_str()).unwrap();

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key_from_a, messages.as_slice())
                .unwrap();
        let mut old_chat: Vec<String> =
            serde_json::from_str(&plaintext_msgs).unwrap();

        let (_chat, _sender_vec) = open_msg(old_chat.clone());
        old_chat
    };

    tokio::time::sleep(Duration::from_millis(6000)).await;

    println!(" ********************** send msg start ********************** ");

    // let msg_u1_to_u2 = format!("Hello, {}", USER_2);
    let msg_u1_to_u2 = format!("Good morning!");
    let (_, u1_pk) = user1.get_sk_pk();

    let msg_pk = vec![msg_u1_to_u2, u1_pk];
    let serialized_msg = vec_serialize(&msg_pk);

    old_chat.push(serialized_msg);
    let chat_vec_str = vec_serialize(&old_chat);

    let ciphertext =
        sak_crypto::aes_encrypt(&aes_key_from_a, chat_vec_str.as_bytes())
            .unwrap();

    let ciphertext_str = vec_serialize(&ciphertext);

    let mut arg = HashMap::with_capacity(10);
    arg.insert(String::from(ARG_CH_ID), DUMMY_CHANNEL_ID_1.to_string());
    arg.insert(String::from(ARG_SERIALIZED_INPUT), ciphertext_str);

    let req_type = String::from("send_msg");

    let json_response =
        send_tx_pour(ctr_addr.clone(), req_type, arg).await.unwrap();

    let result = json_response.result.unwrap();

    println!(" Send_msg result: {:?}", result);

    tokio::time::sleep(Duration::from_millis(6000)).await;

    println!(" ********************** get_msgs start ********************** ");

    {
        let req = {
            let mut arg = HashMap::with_capacity(2);
            arg.insert(String::from(ARG_CH_ID), DUMMY_CHANNEL_ID_1.to_string());

            let req_type = "get_msgs".to_string();

            CtrRequest {
                req_type,
                arg,
                ctr_call_type: CtrCallType::Query,
            }
        };

        let json_response = call_contract(ctr_addr.clone(), req).await.unwrap();
        let query_res = json_response.result.unwrap().result;

        let messages: Vec<u8> =
            serde_json::from_str(query_res.as_str()).unwrap();

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key_from_a, messages.as_slice())
                .unwrap();

        let msgs: Vec<String> = serde_json::from_str(&plaintext_msgs).unwrap();
        let (_chat, _sender_vec) = open_msg(msgs);
    };
}
