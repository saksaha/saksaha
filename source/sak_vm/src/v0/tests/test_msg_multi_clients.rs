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

    let a_sig_str = vec_serialize(&a_sign_key_vec);

    let a_verfying_key = a_sign_key.verifying_key();
    let a_verfying_key_str = vk_serialize(a_verfying_key);
    assert_eq!(a_pk_str, a_verfying_key_str);

    println!("a_pk :{:?}", a_pk_str);
    println!("b_pk :{:?}", b_pk_str);

    // plaintext = chain_id + a_pk + a_sig
    let plaintext: Vec<String> = vec![a_pk_str, a_sig_str];
    let plaintext_str = vec_serialize(&plaintext);

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

fn check_channel(
    my_pk: PublicKey,
    my_sk: SecretKey,
    storage: Storage,
) -> ([u8; 32], String) {
    let my_pk_str = pk_serialize(my_pk);
    let input_serialized = storage.get(&my_pk_str).unwrap();
    let [eph_pk_str, ch_id, pk_sig_encrypted, open_ch_empty]: [String; 4] =
        serde_json::from_str(&input_serialized.as_str()).unwrap();

    let aes_key = {
        let eph_pk_bytes_vec: Vec<u8> =
            serde_json::from_str(&eph_pk_str).unwrap();
        let eph_pk =
            PublicKey::from_sec1_bytes(eph_pk_bytes_vec.as_slice()).unwrap();

        let aes_key = sak_crypto::derive_aes_key(my_sk, eph_pk);

        let ciphertext_empty: Vec<u8> =
            serde_json::from_str(&open_ch_empty).unwrap();

        let plaintext_empty =
            sak_crypto::aes_decrypt(&aes_key, ciphertext_empty.as_slice())
                .unwrap();

        let empty_chat: Vec<String> =
            serde_json::from_str(&plaintext_empty).unwrap();

        assert_eq!(0, empty_chat.len());

        aes_key
    };

    // 2-2. verify who opened the channel() from open_ch_src_pk
    {
        let ciphertext_pk_sig: Vec<u8> =
            serde_json::from_str(&pk_sig_encrypted).unwrap();

        let plaintext_pk_sig: String =
            sak_crypto::aes_decrypt(&aes_key, ciphertext_pk_sig.as_slice())
                .unwrap();
        let [ch_maker_pk_expected, ch_maker_sig]: [String; 2] =
            serde_json::from_str(&plaintext_pk_sig).unwrap();

        let sig_bytes_vec: Vec<u8> =
            serde_json::from_str(&ch_maker_sig).unwrap();
        let ch_maker_sign_key = SigningKey::from_bytes(&sig_bytes_vec).unwrap();

        let ch_maker_pk_from_vk = ch_maker_sign_key.verifying_key();
        let ch_maker_pk_from_sign = serde_json::to_string(
            ch_maker_pk_from_vk.to_encoded_point(false).as_bytes(),
        )
        .unwrap();

        assert_eq!(ch_maker_pk_expected, ch_maker_pk_from_sign);
    }

    (aes_key, ch_id)
}

fn send_msg(
    msg: String,
    pk: PublicKey,
    ch_id: String,
    aes_key: [u8; 32],
    storage: Storage,
    vm: &VM,
) -> (Storage, Vec<String>) {
    // send the message from B to A
    let (state_send_msg, new_chat) = {
        let msgs_serialized = storage.get(&ch_id).unwrap();
        let ciphertext_msgs: Vec<u8> =
            serde_json::from_str(msgs_serialized.as_str()).unwrap();

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key, ciphertext_msgs.as_slice())
                .unwrap();

        let mut old_chat: Vec<String> =
            serde_json::from_str(&plaintext_msgs).unwrap();

        let my_pk_str = pk_serialize(pk);
        let msg_pk = vec![msg, my_pk_str.clone()];
        let serialized_msg = vec_serialize(&msg_pk);

        old_chat.push(serialized_msg);
        let chat_vec_str = vec_serialize(&old_chat);

        let ciphertext =
            sak_crypto::aes_encrypt(&aes_key, chat_vec_str.as_bytes()).unwrap();

        let ciphertext_str = vec_serialize(&ciphertext);

        let mut arg = HashMap::with_capacity(10);
        arg.insert(String::from(ARG_CH_ID), ch_id);
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

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key, ciphertext_msgs.as_slice())
                .unwrap();

        let msgs: Vec<String> = serde_json::from_str(&plaintext_msgs).unwrap();

        assert_eq!(old_chat, msgs);
        (state_send_msg, old_chat)
    };
    (state_send_msg, new_chat)
}

fn pk_serialize(input: PublicKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}

fn vk_serialize(input: VerifyingKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}

fn vec_serialize<T>(input: &T) -> String
where
    T: serde::ser::Serialize,
{
    let ret = serde_json::to_string(input).unwrap();
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

        let a_pk_sig_encrypted = vec_serialize(&ciphertext);

        let empty_chat: Vec<String> = vec![];
        let empty_chat_str = vec_serialize(&empty_chat);
        let ciphertext_empty =
            sak_crypto::aes_encrypt(&aes_key_from_a, empty_chat_str.as_bytes())
                .unwrap();
        let open_ch_empty = vec_serialize(&ciphertext_empty);

        (a_pk_sig_encrypted, open_ch_empty, aes_key_from_a)
    };

    /*  ********************************************************************* */
    // 1. Execute the open_channel that A makes a channel between A and B
    let (request, storage) = {
        // open_ch_input = [eph_pk_str, ch_id, open_ch_src, open_ch_empty]
        let open_ch_input = {
            let open_ch_input: Vec<String> =
                vec![eph_pk_str, ch_id, a_pk_sig_encrypted, open_ch_empty];

            vec_serialize(&open_ch_input)
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

    let (state_open_channel,) = {
        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(request, storage.clone());

        let state_invoked = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("failed to invoke contract : {}", err),
        };

        let state_open_channel: Storage =
            serde_json::from_str(state_invoked.as_str()).unwrap();

        let input_serialized = state_open_channel.get(&b_pk_str).unwrap();
        let [_eph_pk_str, got_ch_id, _a_pk_sig_encrypted, _open_ch_empty]: [String;
            4] = serde_json::from_str(&input_serialized.as_str()).unwrap();

        assert_eq!(DUMMY_CHANNEL_ID_1, got_ch_id);
        (state_open_channel,)
    };

    /*  ********************************************************************* */
    // 2. Execute the send_msg function from B to A
    // check whether the message sender knows the SS or not, and verify who opened the channel
    let (aes_key_from_b, ch_id) =
        check_channel(b_pk, b_sk, state_open_channel.clone());

    // send the message from B to A
    let msgs = vec![
        String::from("Hello, A"),
        String::from("B, welcome to saksaha!"),
    ];

    let (state_send_msg, new_chat) = send_msg(
        msgs[0].clone(),
        b_pk,
        ch_id.clone(),
        aes_key_from_b.clone(),
        state_open_channel,
        &vm,
    );

    /*  ********************************************************************* */
    // 3. User A reads the message sent by B
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
        let ctr_fn = CtrFn::Query(request, state_send_msg.clone());

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

            msg_vec.push(msg);
            pk_vec.push(pk);
        }

        assert_eq!(new_chat, msgs);
    }

    /*  ********************************************************************* */
    // 4. User A replies to B, and shows the chat between A & B
    let (_state_send_msg_2, new_chat) = send_msg(
        msgs[1].clone(),
        a_pk,
        ch_id.clone(),
        aes_key_from_a.clone(),
        state_send_msg,
        &vm,
    );
    let mut msg_vec = Vec::new();
    let mut pk_vec = Vec::new();

    for (i, item) in new_chat.clone().iter().enumerate() {
        let (msg, pk): (String, String) = serde_json::from_str(&item).unwrap();

        println!("\n MSG Sender {:?} \nsays: {:?}", pk, msg);
        msg_vec.push(msg.clone());
        pk_vec.push(pk);
        assert_eq!(msgs[i], msg);
    }
}
