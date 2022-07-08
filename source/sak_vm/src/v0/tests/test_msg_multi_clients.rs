use crate::{CtrFn, VM};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_crypto::{PublicKey, SecretKey, ToEncodedPoint};
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

    let plaintext = String::from("hello");
    let ch_id = String::from("ch_1");

    let storage = Storage::new();

    (
        a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, plaintext, ch_id, storage,
    )
}

#[tokio::test(flavor = "multi_thread")]
async fn test_multi_clients_chat() {
    sak_test_utils::init_test_log();
    sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

    let vm = VM::init().expect("VM should be initiated");

    let (a_sk, a_pk, b_sk, b_pk, eph_sk, eph_pk, plaintext, ch_id, storage) =
        make_test_context();

    let open_ch_msg = {
        let aes_key = sak_crypto::derive_aes_key(eph_sk, b_pk);

        let mut ciphertext =
            sak_crypto::aes_encrypt(&aes_key, plaintext.as_bytes()).unwrap();

        let mut msg = Vec::new();

        msg.extend_from_slice(eph_pk.to_encoded_point(false).as_bytes());
        msg.append(&mut ciphertext);

        serde_json::to_string(&msg).unwrap()
    };

    let b_pk_str =
        serde_json::to_string(b_pk.to_encoded_point(false).as_bytes()).unwrap();

    let (request, storage) = {
        let mut arg = HashMap::with_capacity(2);
        arg.insert(String::from("dst_pk"), b_pk_str);
        arg.insert(String::from("msg"), open_ch_msg);

        let req = Request {
            req_type: String::from("open_ch"),
            arg,
            ctr_call_type: CtrCallType::Execute,
        };

        // let storage = make_dummy_storage(&dummy_messeges);
        (req, storage)
    };

    let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
    let ctr_fn = CtrFn::Execute(request, storage);

    let state_serialized = match vm.invoke(ctr_wasm, ctr_fn) {
        Ok(s) => s,
        Err(err) => panic!("faeild to invoke contract : {}", err),
    };

    // let chats_state: Storage =
    //     serde_json::from_str(state_serialized.as_str()).unwrap();

    // let channel_id = chats_state.get(&her_pk).unwrap();

    // println!("expected channel id : {:?}", DUMMY_CHANNEL_ID_2);
    // println!("updated channel id: {:?}", channel_id);

    // assert_eq!(DUMMY_CHANNEL_ID_2, channel_id);
}
