use super::{
    ARG_CH_ID, ARG_DST_PK, ARG_SERIALIZED_INPUT, DUMMY_CHANNEL_ID_1,
    DUMMY_CHANNEL_ID_2, DUMMY_CHANNEL_ID_3, ENVELOPE_CONTRACT,
    INIT_CHANNEL_ID_1, STORAGE_CAP,
};
use sak_contract_std::{CtrCallType, Request, Storage};
use sak_vm::{CtrFn, VM};
use std::collections::HashMap;

pub(crate) struct OpenChInput {
    eph_pk: String,
    ch_id: String,
    sign: String,
    chat: String,
}

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

    let key = String::from(get_her_pk());

    let input: Vec<String> = vec![
        String::default(),
        DUMMY_CHANNEL_ID_1.to_string(),
        String::default(),
        String::default(),
    ];

    let input = serde_json::to_string(&input).unwrap();

    let input_vec: Vec<String> = vec![input];

    let value = serde_json::to_string(&input_vec).unwrap();

    ret.insert(key, value);

    ret
}

fn make_dummy_open_ch_input() -> OpenChInput {
    OpenChInput {
        eph_pk: String::default(),
        ch_id: DUMMY_CHANNEL_ID_2.to_string(),
        sign: String::default(),
        chat: String::default(),
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_messenger_init() {
    // init();
    let vm = VM::init().expect("VM should be initiated");

    let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
    let ctr_fn = CtrFn::Init;

    let messenger_states_invoked = vm
        .invoke(ctr_wasm, ctr_fn)
        .expect("channels should be obtained");

    let messenger_states: Storage =
        serde_json::from_str(messenger_states_invoked.as_str()).unwrap();

    let messages: Vec<String> =
        serde_json::from_str(messenger_states.get(INIT_CHANNEL_ID_1).unwrap())
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
        let mut args = HashMap::with_capacity(1);
        args.insert(String::from(ARG_CH_ID), String::from(DUMMY_CHANNEL_ID_1));

        Request {
            req_type: "get_msgs".to_string(),
            args,
            ctr_call_type: CtrCallType::Query,
        }
    };

    {
        let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
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
}

#[tokio::test(flavor = "multi_thread")]
async fn test_messenger_get_ch_list() {
    let vm = VM::init().expect("VM should be initiated");

    let her_pk = get_her_pk();

    let dummy_messeges = get_multi_messages();

    let (request, storage) = {
        let mut args = HashMap::with_capacity(2);
        args.insert(String::from(ARG_DST_PK), her_pk.clone());

        let req = Request {
            req_type: String::from("get_ch_list"),
            args,
            ctr_call_type: CtrCallType::Query,
        };
        let storage = make_dummy_storage(&dummy_messeges);
        (req, storage)
    };

    {
        let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
        let ctr_fn = CtrFn::Query(request, storage);

        let ch_list_serialized = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("failed to invoke contract : {}", err),
        };

        let open_ch_data_vec: Vec<String> =
            serde_json::from_str(&ch_list_serialized).unwrap();

        println!("expected channel id : {:?}", vec![DUMMY_CHANNEL_ID_1]);
        println!("updated channel id: {:?}", open_ch_data_vec);

        assert_eq!(vec![DUMMY_CHANNEL_ID_1], open_ch_data_vec);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_messenger_open_channel() {
    let vm = VM::init().expect("VM should be initiated");

    let new_pk = "abcdef".to_string();

    let dummy_messeges = get_multi_messages();

    let OpenChInput {
        eph_pk,
        ch_id,
        sign,
        chat,
    } = make_dummy_open_ch_input();

    let input = {
        let input = vec![eph_pk, ch_id, sign, chat];

        let input_str = serde_json::to_string(&input).unwrap();

        input_str
    };

    let (request, storage) = {
        let mut args = HashMap::with_capacity(2);
        args.insert(String::from(ARG_DST_PK), new_pk.clone());
        args.insert(String::from(ARG_SERIALIZED_INPUT), input);

        let req = Request {
            req_type: String::from("open_channel"),
            args,
            ctr_call_type: CtrCallType::Execute,
        };
        let storage = make_dummy_storage(&dummy_messeges);
        (req, storage)
    };

    {
        let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
        let ctr_fn = CtrFn::Execute(request, storage);

        let state_serialized = match vm.invoke(ctr_wasm, ctr_fn) {
            Ok(s) => s,
            Err(err) => panic!("faeild to invoke contract : {}", err),
        };

        let chats_state: Storage =
            serde_json::from_str(state_serialized.as_str()).unwrap();

        let open_ch_serialized = chats_state.get(&new_pk).unwrap();
        let open_ch_vec: Vec<String> =
            serde_json::from_str(open_ch_serialized).unwrap();

        let open_ch: Vec<String> =
            serde_json::from_str(&open_ch_vec[0]).unwrap();

        println!("expected channel id : {:?}", DUMMY_CHANNEL_ID_2);
        println!("updated channel id: {:?}", open_ch);

        let ch_id = &open_ch[1];

        assert_eq!(DUMMY_CHANNEL_ID_2, ch_id);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_messenger_send_msg() {
    let vm = VM::init().expect("VM should be initiated");

    let dummy_messeges = get_multi_messages();

    let expected_msg = get_single_message();

    let (request, storage) = {
        let mut args = HashMap::with_capacity(2);
        args.insert(String::from(ARG_CH_ID), String::from(DUMMY_CHANNEL_ID_3));
        args.insert(String::from(ARG_SERIALIZED_INPUT), expected_msg.clone());

        let req = Request {
            req_type: String::from("send_msg"),
            args,
            ctr_call_type: CtrCallType::Execute,
        };
        let storage = make_dummy_storage(&dummy_messeges);
        (req, storage)
    };

    {
        let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
        let ctr_fn = CtrFn::Execute(request, storage);

        let chats_state_serialized = vm
            .invoke(ctr_wasm, ctr_fn)
            .expect("State should be obtained");

        let chats_state: Storage =
            serde_json::from_str(chats_state_serialized.as_str()).unwrap();

        let msg = chats_state.get(DUMMY_CHANNEL_ID_3).unwrap();
        println!("expected msgs: {:?}", expected_msg);
        println!("updated msgs: {:?}", msg);

        assert_eq!(&expected_msg, msg);
    };
}
