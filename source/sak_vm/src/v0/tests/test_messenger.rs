#[cfg(test)]
mod test {
    use crate::v0::tests::test_msg_multi_clients::*;
    use crate::*;
    use std::collections::HashMap;

    use crate::{CtrFn, VM};
    use sak_contract_std::{CtrCallType, Request, Storage};

    fn get_multi_messages() -> Vec<String> {
        vec![
            String::from("Hi, there"),
            String::from("This is a secret message"),
        ]
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_init() {
        sak_test_utils::init_test_log();
        sak_test_utils::init_test_config(&vec![String::from("test")]).unwrap();

        let vm = VM::init().expect("VM should be initiated");

        let ctr_wasm = include_bytes!("../sak_ctr_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Init;

        let messenger_states_invoked = vm
            .invoke(ctr_wasm, ctr_fn)
            .expect("channels should be obtained");

        let messenger_states: Storage =
            serde_json::from_str(messenger_states_invoked.as_str()).unwrap();

        let messages: Vec<String> = serde_json::from_str(
            messenger_states.get(DUMMY_CHANNEL_ID_1).unwrap(),
        )
        .unwrap();

        let messages_expected = get_multi_messages();

        println!("messages expected: {:?}", messages_expected);
        println!("messages acquired: {:?}", messages);

        assert_eq!(messages_expected, messages);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_open_channel_and_get_ch_list() {
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
                let ciphertext = sak_crypto::aes_encrypt(
                    &aes_key_from_a,
                    credential.as_bytes(),
                )
                .unwrap();
                vec_serialize(&ciphertext)
            };

            let empty_chat: Vec<String> = vec![];
            let empty_chat_str = vec_serialize(&empty_chat);
            let ciphertext_empty = sak_crypto::aes_encrypt(
                &aes_key_from_a,
                empty_chat_str.as_bytes(),
            )
            .unwrap();
            let open_ch_empty = vec_serialize(&ciphertext_empty);

            (a_credential_encrypted, open_ch_empty, aes_key_from_a)
        };

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

        let vm = VM::init().expect("VM should be initiated");

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

        test_get_ch_list(b_pk, state_after_open_ch.clone(), &vm);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_send_msg_and_get_msgs() {
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
                let ciphertext = sak_crypto::aes_encrypt(
                    &aes_key_from_a,
                    credential.as_bytes(),
                )
                .unwrap();
                vec_serialize(&ciphertext)
            };

            let empty_chat: Vec<String> = vec![];
            let empty_chat_str = vec_serialize(&empty_chat);
            let ciphertext_empty = sak_crypto::aes_encrypt(
                &aes_key_from_a,
                empty_chat_str.as_bytes(),
            )
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

        let msg_b_to_a = String::from("Hello, A");

        let (state_send_msg, new_chat) = send_msg(
            &msg_b_to_a,
            b_pk,
            ch_id.clone(),
            aes_key_from_b.clone(),
            state_after_open_ch,
            &vm,
        );

        // 3. User A check A's channl list and reads the message sent by B
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

            let msgs: Vec<String> =
                serde_json::from_str(&plaintext_msgs).unwrap();

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
    }
}
