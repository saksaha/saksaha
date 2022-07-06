#[cfg(test)]
mod test {

    use std::collections::HashMap;
    const STORAGE_CAP: usize = 100;
    const DUMMY_CHANNEL_ID_1: &str = "dummy_channel_1";
    const DUMMY_CHANNEL_ID_2: &str = "dummy_channel_2";

    use crate::{CtrFn, VM};
    use env_logger::init;
    use sak_contract_std::{CtrCallType, Request, Storage};

    fn get_single_message() -> String {
        String::from("Hello! I belong to saksaha")
    }

    fn get_multi_messages() -> Vec<String> {
        vec![
            String::from("Hi, there"),
            String::from("This is a secret message"),
        ]
    }

    fn get_her_pk() -> String {
        String::from("her_pk12345")
    }

    fn make_dummy_storage(msgs: &Vec<String>) -> Storage {
        let mut ret = Storage::with_capacity(STORAGE_CAP);

        let key = String::from(DUMMY_CHANNEL_ID_1);
        let value = serde_json::to_string(&msgs).unwrap();

        ret.insert(key, value);

        ret
    }

    fn make_empty_storage() -> Storage {
        let ret = Storage::with_capacity(STORAGE_CAP);

        ret
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_init() {
        init();
        let vm = VM::init().expect("VM should be initiated");

        let ctr_wasm = include_bytes!("../sak_ctrt_messenger.wasm").to_vec();
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
    async fn test_messenger_get_msgs() {
        let vm = VM::init().expect("VM should be initiated");

        let test_dummy_messege = get_multi_messages();

        let messages_state = make_dummy_storage(&test_dummy_messege);

        let request = {
            let mut arg = HashMap::with_capacity(1);
            arg.insert(
                String::from("channel_id"),
                String::from(DUMMY_CHANNEL_ID_1),
            );

            Request {
                req_type: "get_msgs".to_string(),
                arg,
                ctr_call_type: CtrCallType::Query,
            }
        };

        let ctr_wasm = include_bytes!("../sak_ctrt_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Query(request, messages_state);

        let messages_from_query = vm
            .invoke(ctr_wasm, ctr_fn)
            .expect("message should be obtained");

        let messages: Vec<String> =
            serde_json::from_str(messages_from_query.as_str()).unwrap();

        println!("messages expected: {:?}", test_dummy_messege);
        println!("messages acquired: {:?}", messages);

        assert_eq!(test_dummy_messege, messages);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_open_channel() {
        let vm = VM::init().expect("VM should be initiated");

        let her_pk = get_her_pk();

        let dummy_messeges = get_multi_messages();

        let (request, storage) = {
            let mut arg = HashMap::with_capacity(2);
            arg.insert(String::from("her_pk"), her_pk.clone());
            arg.insert(
                String::from("channel_id"),
                String::from(DUMMY_CHANNEL_ID_2),
            );

            let req = Request {
                req_type: String::from("open_channel"),
                arg,
                ctr_call_type: CtrCallType::Execute,
            };
            let storage = make_dummy_storage(&dummy_messeges);
            (req, storage)
        };

        let ctr_wasm = include_bytes!("../sak_ctrt_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(request, storage);

        let state_serialized = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("faeild to invoke contract : {}", err),
        };

        let chats_state: Storage =
            serde_json::from_str(state_serialized.as_str()).unwrap();

        let channel_id = chats_state.get(&her_pk).unwrap();

        println!("expected channel id : {:?}", DUMMY_CHANNEL_ID_2);
        println!("updated channel id: {:?}", channel_id);

        assert_eq!(DUMMY_CHANNEL_ID_2, channel_id);
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn test_messenger_send_msg() {
        let vm = VM::init().expect("VM should be initiated");

        let mut dummy_messeges = get_multi_messages();

        let (request, storage) = {
            let mut arg = HashMap::with_capacity(2);
            arg.insert(
                String::from("channel_id"),
                String::from(DUMMY_CHANNEL_ID_1),
            );
            arg.insert(String::from("message"), get_single_message());

            let req = Request {
                req_type: String::from("send_msg"),
                arg,
                ctr_call_type: CtrCallType::Execute,
            };
            let storage = make_dummy_storage(&dummy_messeges);
            (req, storage)
        };

        let ctr_wasm = include_bytes!("../sak_ctrt_messenger.wasm").to_vec();
        let ctr_fn = CtrFn::Execute(request, storage);

        let chats_state_serialized = vm
            .invoke(ctr_wasm, ctr_fn)
            .expect("State should be obtained");

        let chats_state: Storage =
            serde_json::from_str(chats_state_serialized.as_str()).unwrap();

        let msgs_serialized = chats_state.get(DUMMY_CHANNEL_ID_1).unwrap();

        let msgs: Vec<String> =
            serde_json::from_str(msgs_serialized.as_str()).unwrap();

        dummy_messeges.push(get_single_message());

        println!("expected msgs: {:?}", dummy_messeges);
        println!("updated msgs: {:?}", msgs);

        assert_eq!(dummy_messeges, msgs);
    }
}
