use crate::*;
use crate::{CtrFn, VM};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_crypto::{
    PublicKey, SakKey, SecretKey, SigningKey, ToEncodedPoint, VerifyingKey,
};
use std::collections::HashMap;

pub(crate) fn make_test_context() -> (
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    SecretKey,
    PublicKey,
    String,
    String,
    Storage,
    VM,
) {
    let vm = VM::init().expect("VM should be initiated");

    let (a_sk, a_pk) = SakKey::generate();
    let (b_sk, b_pk) = SakKey::generate();
    let (eph_sk, eph_pk) = SakKey::generate();

    let ch_id = String::from(DUMMY_CHANNEL_ID_1);

    let a_pk_str = pk_serialize(a_pk);
    let b_pk_str = pk_serialize(b_pk);

    let a_sig_str = {
        let a_sign_key = SigningKey::from(&a_sk);
        let a_sign_key_vec = a_sign_key.to_bytes().to_vec();
        vec_serialize(&a_sign_key_vec)
    };

    println!("a_pk :{:?}", a_pk_str);
    println!("b_pk :{:?}", b_pk_str);

    let credential = {
        let v: Vec<String> = vec![a_pk_str, a_sig_str];
        vec_serialize(&v)
    };

    let storage = Storage::new();

    (
        a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, credential, ch_id, storage, vm,
    )
}

pub(crate) fn check_channel(
    my_pk: PublicKey,
    my_sk: SecretKey,
    storage: Storage,
) -> ([u8; 32], String) {
    let my_pk_str =
        serde_json::to_string(my_pk.to_encoded_point(false).as_bytes())
            .unwrap();

    let open_ch_data_vec = storage.get(&my_pk_str).unwrap();

    let [mut eph_pk_str_vec, mut ch_id_vec,mut  pk_sig_encrypted_vec,mut  open_ch_empty_vec]: [Vec<String>;
        4] = [vec![], vec![], vec![], vec![]];

    let (eph_pk_str, ch_id, pk_sig_encrypted, open_ch_empty) = {
        let open_ch_data_vec: Vec<String> =
            serde_json::from_str(&open_ch_data_vec).unwrap();
        for data in open_ch_data_vec {
            let res: Vec<String> =
                serde_json::from_str(&data.as_str()).unwrap();
            eph_pk_str_vec.push(res[0].clone());
            ch_id_vec.push(res[1].clone());
            pk_sig_encrypted_vec.push(res[2].clone());
            open_ch_empty_vec.push(res[3].clone());
        }

        let idx = match ch_id_vec.iter().position(|r| r == DUMMY_CHANNEL_ID_1) {
            Some(o) => o,
            _ => panic!("ch_id should be stored"),
        };

        (
            eph_pk_str_vec[idx].clone(),
            ch_id_vec[idx].clone(),
            pk_sig_encrypted_vec[idx].clone(),
            open_ch_empty_vec[idx].clone(),
        )
    };

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

        assert_eq!(0, empty_chat.len(), "chat has to be empty until now");

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

pub(crate) fn test_get_ch_list(
    pk: PublicKey,
    storage: Storage,
    vm: &VM,
) -> Vec<String> {
    let pk_str =
        serde_json::to_string(pk.to_encoded_point(false).as_bytes()).unwrap();

    let request = {
        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from(ARG_DST_PK), pk_str.clone());

        let req = Request {
            req_type: String::from("get_ch_list"),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };
        req
    };

    let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
    let ctr_fn = CtrFn::Query(request, storage.clone());

    let ch_list_serialized = match vm.invoke(ctr_wasm, ctr_fn) {
        Ok(s) => s,
        Err(err) => panic!("failed to invoke contract : {}", err),
    };

    let ch_list: Vec<String> =
        serde_json::from_str(&ch_list_serialized).unwrap();

    assert_eq!(vec![DUMMY_CHANNEL_ID_1], ch_list);
    ch_list
}

pub(crate) fn send_msg(
    msg: &String,
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
        let msg_pk = vec![msg, &my_pk_str];
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

pub fn pk_serialize(input: PublicKey) -> String {
    let ret = serde_json::to_string(input.to_encoded_point(false).as_bytes())
        .unwrap();
    ret
}

pub fn vec_serialize<T>(input: &T) -> String
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

    let (
        _a_sk,
        a_pk,
        b_sk,
        b_pk,
        eph_sk,
        eph_pk,
        credential,
        ch_id,
        storage,
        vm,
    ) = make_test_context();

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

    /*  ******************************************************************** */
    // 1. Request open_channel A -> B
    let (open_ch_req, storage) = {
        // open_ch_input = [eph_pk_str, ch_id, open_ch_src, open_ch_empty]
        let open_ch_input = {
            let open_ch_input: Vec<String> =
                vec![eph_pk_str, ch_id, a_pk_sig_encrypted, open_ch_empty];

            serde_json::to_string(&open_ch_input).unwrap()
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

    let state_after_open_ch = {
        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(open_ch_req, storage.clone());

        let state_invoked = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("failed to invoke contract : {}", err),
        };

        let state_open_channel: Storage =
            serde_json::from_str(state_invoked.as_str()).unwrap();

        let open_ch_data_vec = state_open_channel.get(&b_pk_str).unwrap();

        let mut ch_id_vec = vec![];

        let ch_list = {
            let open_ch_data_vec: Vec<String> =
                serde_json::from_str(&open_ch_data_vec.as_str()).unwrap();
            for data in &open_ch_data_vec {
                let res: Vec<String> = serde_json::from_str(data).unwrap();
                ch_id_vec.push(res[1].clone());
            }

            ch_id_vec
        };

        assert_eq!(vec![DUMMY_CHANNEL_ID_1], ch_list);

        state_open_channel
    };

    /*  ********************************************************************* */
    // 2. Request get_ch_list and send_msg B -> A
    let (aes_key_from_b, ch_id) =
        check_channel(b_pk, b_sk, state_after_open_ch.clone());

    test_get_ch_list(b_pk, state_after_open_ch.clone(), &vm);

    let msg_b_to_a = String::from("Hello, A");

    let (state_send_msg, new_chat) = send_msg(
        &msg_b_to_a,
        b_pk,
        ch_id.clone(),
        aes_key_from_b.clone(),
        state_after_open_ch,
        &vm,
    );

    /*  ********************************************************************* */
    // 3. Request get_msgs
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

        println!("Messages in the chat, encrypted: {:?}", messages);

        let plaintext_msgs =
            sak_crypto::aes_decrypt(&aes_key_from_a, messages.as_slice())
                .unwrap();

        let msgs: Vec<String> = serde_json::from_str(&plaintext_msgs).unwrap();

        println!("Messages in the chat, decrypted: {:?}", msgs);

        let mut msg_vec = Vec::new();
        let mut pk_vec = Vec::new();

        for msg in &msgs {
            let (msg, pk): (String, String) =
                serde_json::from_str(msg).unwrap();

            msg_vec.push(msg);
            pk_vec.push(pk);
        }

        assert_eq!(new_chat, msgs);
    }

    /*  ********************************************************************* */
    // 4. User A replies to B, and reads the chat between A & B
    let msg_a_to_b = String::from("B, welcome to saksaha!");

    let (_state_send_msg_2, new_chat) = send_msg(
        &msg_a_to_b,
        a_pk,
        ch_id.clone(),
        aes_key_from_a.clone(),
        state_send_msg,
        &vm,
    );
    let mut msg_vec = Vec::new();
    let mut pk_vec = Vec::new();

    let msgs = [msg_b_to_a, msg_a_to_b];

    for (i, item) in (&new_chat).iter().enumerate() {
        let (msg, pk): (String, String) = serde_json::from_str(&item).unwrap();

        println!("\n MSG Sender {:?} \nsays: {:?}", pk, msg);

        msg_vec.push(msg.clone());
        pk_vec.push(pk);

        assert_eq!(msgs[i], msg);
    }
}
