use crate::*;
use crate::{CtrFn, VM};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_crypto::{
    PublicKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};
use std::collections::HashMap;

fn make_test_context() -> (
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    String,
    String,
    Storage,
) {
    let (a_sk, a_pk) = sak_crypto::generate_key_pair();
    let (b_sk, b_pk) = sak_crypto::generate_key_pair();
    let (eph_sk, eph_pk) = sak_crypto::generate_key_pair();

    let ch_id = String::from(DUMMY_CHANNEL_ID_1);

    let a_pk_str = pk_serialize(a_pk);
    let b_pk_str = pk_serialize(b_pk);

    let a_sign_key = SigningKey::from(&a_sk);

    let a_sign_key_vec = a_sign_key.to_bytes().to_vec();

    let a_sig_str = vec_u8_serialize(&a_sign_key_vec);

    let a_verfying_key = a_sign_key.verifying_key();
    let a_verfying_key_str = vk_serialize(a_verfying_key);
    assert_eq!(a_pk_str, a_verfying_key_str);

    println!("a_pk :{:?}", a_pk_str);
    println!("b_pk :{:?}", b_pk_str);

    // plaintext = chain_id + a_pk + a_sig
    let plaintext: Vec<String> = vec![a_pk_str, a_sig_str];
    let plaintext_str = vec_str_serialize(&plaintext);

    let storage = Storage::new();

    (
        a_sk,
        a_pk,
        b_sk,
        b_pk,
        eph_sk,
        eph_pk,
        plaintext_str,
        ch_id,
        storage,
    )
}

fn pk_serialize(input: PublicKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}

fn vec_u8_serialize(input: &Vec<u8>) -> String {
    let ret = serde_json::to_string(input).unwrap();
    ret
}

fn vec_str_serialize(input: &Vec<String>) -> String {
    let ret = serde_json::to_string(input).unwrap();
    ret
}

fn vk_serialize(input: VerifyingKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}
#[tokio::test(flavor = "multi_thread")]
async fn test_multi_clients_chat() {
    /* 0. Init and make a test context */
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let vm = VM::init().expect("VM should be initiated");

    let (_a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, plaintext, ch_id, storage) =
        make_test_context();

    let eph_pk_str = pk_serialize(eph_pk);

    let b_pk_str = pk_serialize(b_pk);

    let (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a) = {
        let aes_key_from_a = sak_crypto::derive_aes_key(eph_sk, b_pk);

        let ciphertext =
            sak_crypto::aes_encrypt(&aes_key_from_a, plaintext.as_bytes())
                .unwrap();

        let a_pk_sig_encrypted = vec_u8_serialize(&ciphertext);

        let empty_chat: Vec<String> = vec![];
        let empty_chat_str = vec_str_serialize(&empty_chat);
        let ciphertext_empty =
            sak_crypto::aes_encrypt(&aes_key_from_a, empty_chat_str.as_bytes())
                .unwrap();
        let open_ch_empty = vec_u8_serialize(&ciphertext_empty);

        (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a)
    };

    /*  ********************************************************************* */
    // 1. Execute the open_channel that A makes a channel between A and B
    let (request, storage) = {
        // open_ch_input = [eph_pk_str, ch_id, open_ch_src, open_ch_empty]
        let open_ch_input = {
            let open_ch_input: Vec<String> =
                vec![eph_pk_str, ch_id, a_pk_sig_encrypted, open_ch_empty];

            vec_str_serialize(&open_ch_input)
        };

        let mut arg = HashMap::with_capacity(10);
        arg.insert(String::from(ARG_DST_PK), b_pk_str.clone());
        arg.insert(String::from(ARG_SERIALIZED_INPUT), open_ch_input);

        let req = Request {
            req_type: String::from("open_channel"),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        (req, storage)
    };

    let (eph_pk_str, got_ch_id, a_pk_sig_encrypted, open_ch_empty) = {
        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(request, storage.clone());

        let state_invoked = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("failed to invoke contract : {}", err),
        };

        let state_open_channel: Storage =
            serde_json::from_str(state_invoked.as_str()).unwrap();

        let input_serialized = state_open_channel.get(&b_pk_str).unwrap();
        let [eph_pk_str, got_ch_id, a_pk_sig_encrypted, open_ch_empty]: [String;
            4] = serde_json::from_str(&input_serialized.as_str()).unwrap();

        assert_eq!(DUMMY_CHANNEL_ID_1, got_ch_id);
        (eph_pk_str, got_ch_id, a_pk_sig_encrypted, open_ch_empty)
    };

    /*  ********************************************************************* */
    // 2. Execute the send_msg function from B to A
    // 2-1. check whether the message sender knows the SS or not.
    let (empty_chat, aes_key_from_b) = {
        let eph_pk_bytes_vec: Vec<u8> =
            serde_json::from_str(&eph_pk_str).unwrap();
        let eph_pk =
            PublicKey::from_sec1_bytes(eph_pk_bytes_vec.as_slice()).unwrap();

        let aes_key_from_b = sak_crypto::derive_aes_key(b_sk, eph_pk);

        let ciphertext_empty: Vec<u8> =
            serde_json::from_str(&open_ch_empty).unwrap();

        let plaintext_empty = sak_crypto::aes_decrypt(
            &aes_key_from_b,
            ciphertext_empty.as_slice(),
        )
        .unwrap();

        let empty_chat: Vec<String> =
            serde_json::from_str(&plaintext_empty).unwrap();

        assert_eq!(0, empty_chat.len());

        (empty_chat, aes_key_from_b)
    };

    // 2-2. verify who opened the channel() from open_ch_src_pk
    {
        let ciphertext_a_pk_sig: Vec<u8> =
            serde_json::from_str(&a_pk_sig_encrypted).unwrap();

        let plaintext_a_pk_sig: String = sak_crypto::aes_decrypt(
            &aes_key_from_b,
            ciphertext_a_pk_sig.as_slice(),
        )
        .unwrap();
        let [a_pk_from_cipher, a_sig]: [String; 2] =
            serde_json::from_str(&plaintext_a_pk_sig).unwrap();

        let a_pk_expected = pk_serialize(a_pk);

        // verify a_pk == a_pk_from_cipher
        assert_eq!(a_pk_expected, a_pk_from_cipher);

        let sig_str_bytes_vec: Vec<u8> = serde_json::from_str(&a_sig).unwrap();
        let a_sign_key = SigningKey::from_bytes(&sig_str_bytes_vec).unwrap();

        let a_pk_from_vk = a_sign_key.verifying_key();
        let a_pk_from_sign = serde_json::to_string(
            a_pk_from_vk.to_encoded_point(false).as_bytes(),
        )
        .unwrap();
        // verify a_pk == a_pk_from_sign
        assert_eq!(a_pk_expected, a_pk_from_sign);
    }

    // 2-3. send the message from the sender(B) to the receiver(A)
    let (state_send_msg, new_chat) = {
        let msg = String::from("Hello, A");
        let msg_w_pk = vec![msg, b_pk_str.clone()];
        let serialized_msg = vec_str_serialize(&msg_w_pk);

        let mut new_chat = empty_chat;
        new_chat.push(serialized_msg);
        let chat_vec_str = vec_str_serialize(&new_chat);

        let ciphertext =
            sak_crypto::aes_encrypt(&aes_key_from_b, chat_vec_str.as_bytes())
                .unwrap();

        let ciphertext_str = vec_u8_serialize(&ciphertext);

        let mut arg = HashMap::with_capacity(10);
        arg.insert(String::from(ARG_CH_ID), got_ch_id);
        arg.insert(String::from(ARG_SERIALIZED_INPUT), ciphertext_str);

        let req = Request {
            req_type: String::from("send_msg"),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(req, storage);

        let state_invoked = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("failed to invoke contract : {}", err),
        };

        let state_send_msg: Storage =
            serde_json::from_str(state_invoked.as_str()).unwrap();

        let msgs_serialized = state_send_msg.get(DUMMY_CHANNEL_ID_1).unwrap();
        let ciphertext_msgs: Vec<u8> =
            serde_json::from_str(msgs_serialized.as_str()).unwrap();

        let plaintext_msgs = sak_crypto::aes_decrypt(
            &aes_key_from_b,
            ciphertext_msgs.as_slice(),
        )
        .unwrap();

        let msgs: Vec<String> = serde_json::from_str(&plaintext_msgs).unwrap();

        assert_eq!(new_chat, msgs);
        (state_send_msg, new_chat)
    };

    /*  ********************************************************************* */
    // 3. user A reads the message sent from B
    {
        let request = {
            let mut arg = HashMap::with_capacity(1);
            arg.insert(
                String::from(ARG_CH_ID),
                String::from(DUMMY_CHANNEL_ID_1),
            );

            Request {
                req_type: "get_msgs".to_string(),
                arg,
                ctr_call_type: CtrCallType::Query,
            }
        };

        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Query(request, state_send_msg);

        let messages_from_query = vm
            .invoke(ctr_wasm, ctr_fn)
            .expect("message should be obtained");

        let messages: Vec<u8> =
            serde_json::from_str(messages_from_query.as_str()).unwrap();

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key_from_a, messages.as_slice())
                .unwrap();

        let msgs: Vec<String> = serde_json::from_str(&plaintext_msgs).unwrap();

        let mut msg_vec = Vec::new();
        let mut pk_vec = Vec::new();

        for msg in msgs.clone() {
            let (msg, pk): (String, String) =
                serde_json::from_str(&msg).unwrap();

            println!("Sender {:?} says: {:?}", pk, msg);
            msg_vec.push(msg);
            pk_vec.push(pk);
        }

        assert_eq!(new_chat, msgs);
    }
}
