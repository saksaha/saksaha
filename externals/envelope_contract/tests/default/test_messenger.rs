use super::{
    constants::{
        ARG_CH_ID, ARG_DST_PK, ARG_SERIALIZED_INPUT, DUMMY_CHANNEL_ID_1, DUMMY_CHANNEL_ID_2,
        DUMMY_CHANNEL_ID_3, ENVELOPE_CONTRACT,
    },
    utils::EnvelopeTestUtils,
};
use envelope_contract::{
    request_type::OPEN_CH, Channel, ChannelId, ChatMessage, EncryptedChatMessage, EnvelopeStorage,
    GetChListParams, GetMsgParams, OpenChParams, SendMsgParams,
};
use sak_contract_std::{ContractFn, CtrCallType, CtrRequest, CtrRequestData, Storage};
use sak_credential::CredentialProfile;
use sak_mrs::SakMRS;
use sak_store_accessor::StoreAccessor;
use sak_vm::SakVM;
use sak_vm_interface::{ContractProcess, ContractProcessor};
use std::{collections::HashMap, sync::Arc};

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

fn mock_storage(msgs: &Vec<String>) -> Storage {
    let mut open_ch_reqs = HashMap::new();

    let mut chats = HashMap::<ChannelId, Vec<String>>::new();

    let chat_msg = ChatMessage {
        date: "test_date".to_string(),
        user: get_her_pk(),
        msg: "hello".to_string(),
    };

    chats.insert(
        DUMMY_CHANNEL_ID_1.to_string(),
        vec![serde_json::to_string(&chat_msg).unwrap()],
    );

    let envelope_storage = EnvelopeStorage {
        open_ch_reqs,
        chats,
    };

    serde_json::to_vec(&envelope_storage).unwrap()
}

fn make_mock_open_ch() -> Channel {
    Channel {
        ch_id: DUMMY_CHANNEL_ID_2.to_string(),
        eph_key: String::default(),
        // sig: String::default(),
        initiator_pk: String::default(),
        participants: Vec::<String>::new(),
    }
}

// #[tokio::test(flavor = "multi_thread")]
// async fn test_messenger_get_msgs() {
//     EnvelopeTestUtils::init_test_log();

//     let vm: ContractProcessor = {
//         let v = SakVM::init().expect("VM should be initiated");
//         Box::new(v)
//     };

//     let test_dummy_messege = get_multi_messages();

//     // let messages_state = mock_storage(&test_dummy_messege);

//     let request = {
//         let get_msg_params = GetMsgParams {
//             ch_id: DUMMY_CHANNEL_ID_1.to_string(),
//         };

//         let args = serde_json::to_vec(&get_msg_params).unwrap();

//         CtrRequestData {
//             req_type: "get_msgs".to_string(),
//             args,
//             ctr_call_type: CtrCallType::Query,
//         }
//     };

//     {
//         let mrs_path = "";

//         let mrs = {
//             let m = SakMRS::init(&mrs_path).await.unwrap();
//             m
//         };

//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         // let ctr_fn = ContractFn::Query(request, messages_state);
//         let ctr_fn = ContractFn::Query(request);

//         let receipt = vm
//             .invoke(&ctr_wasm, ctr_fn)
//             .expect("message should be obtained");

//         let result = receipt.result;

//         let chats: Vec<String> = serde_json::from_slice(&result).unwrap();

//         println!("messages expected: {:?}", test_dummy_messege);

//         println!("messages acquired: {:?}", chats);

//         assert_eq!(test_dummy_messege, chats);
//     }
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_messenger_get_ch_list() {
//     EnvelopeTestUtils::init_test_log();

//     let vm = SakVM::init().expect("VM should be initiated");

//     let her_pk = get_her_pk();

//     let dummy_messeges = get_multi_messages();

//     let (request, storage) = {
//         let get_ch_list_params = GetChListParams {
//             dst_pk: her_pk.clone(),
//         };

//         let args = serde_json::to_vec(&get_ch_list_params).unwrap();

//         let req = CtrRequest {
//             req_type: String::from("get_ch_list"),
//             args,
//             ctr_call_type: CtrCallType::Query,
//         };

//         let storage = mock_storage(&dummy_messeges);

//         (req, storage)
//     };

//     {
//         let mrs_path = "";

//         let mrs = {
//             let m = SakMRS::init(&mrs_path).await.unwrap();
//             m
//         };

//         // let store_accessor = {
//         //     let a = StoreAccessor::new(mrs);
//         //     Arc::new(a)
//         // };

//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         let ctr_fn = ContractFn::Query(request, storage);

//         let receipt = vm.invoke(&ctr_wasm, ctr_fn).unwrap();

//         let open_ch_data_vec: Vec<Channel> = serde_json::from_slice(&receipt.result).unwrap();

//         println!("expected channel id : {:?}", vec![DUMMY_CHANNEL_ID_1]);

//         println!("updated channel id: {:?}", open_ch_data_vec);

//         assert_eq!(
//             vec![DUMMY_CHANNEL_ID_1],
//             vec![open_ch_data_vec[1].ch_id.to_owned()]
//         );
//     }
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_messenger_open_channel() {
//     EnvelopeTestUtils::init_test_log();

//     let vm = SakVM::init().expect("VM should be initiated");

//     let credential = CredentialProfile::test_1();

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let mrs_path = { test_dir.join("mrs") };

//     let mrs = SakMRS::init(mrs_path).await.unwrap();

//     // let store_accessor = {
//     //     let a = StoreAccessor::new(mrs);
//     //     Arc::new(a)
//     // };

//     let new_pk = "abcdef".to_string();

//     let dummy_messeges = get_multi_messages();

//     let Channel {
//         ch_id,
//         eph_key,
//         // sig,
//         initiator_pk,
//         participants,
//     } = make_mock_open_ch();

//     let (request, storage) = {
//         let open_ch_params = OpenChParams {
//             dst_pk: new_pk.clone(),
//             open_ch: Channel {
//                 ch_id,
//                 eph_key,
//                 // sig,
//                 initiator_pk,
//                 participants,
//             },
//         };

//         let args = serde_json::to_vec(&open_ch_params).unwrap();

//         let req = CtrRequest {
//             req_type: OPEN_CH.to_string(),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let storage = mock_storage(&dummy_messeges);

//         (req, storage)
//     };

//     {
//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         let ctr_fn = ContractFn::Execute(request, storage);

//         let receipt = vm.invoke(&ctr_wasm, ctr_fn).unwrap();

//         let updated_storage = receipt
//             .updated_storage
//             .ok_or("State needs to be updated, ")
//             .unwrap();

//         let storage: EnvelopeStorage = serde_json::from_slice(&updated_storage).unwrap();

//         println!("expected channel id : {:?}", DUMMY_CHANNEL_ID_2);
//         println!("updated channel id: {:?}", storage);
//     }
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_messenger_send_msg() {
//     EnvelopeTestUtils::init_test_log();

//     let vm = SakVM::init().expect("VM should be initiated");

//     let credential = CredentialProfile::test_1();

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let mrs_path = { test_dir.join("mrs") };

//     let mrs = SakMRS::init(mrs_path).await.unwrap();

//     // let store_accessor = {
//     //     let a = StoreAccessor::new(mrs);
//     //     Arc::new(a)
//     // };

//     let dummy_messeges = get_multi_messages();

//     let expected_msg = get_single_message();

//     let (request, storage) = {
//         let mut args = HashMap::with_capacity(2);
//         args.insert(String::from(ARG_CH_ID), String::from(DUMMY_CHANNEL_ID_3));
//         args.insert(String::from(ARG_SERIALIZED_INPUT), expected_msg.clone());

//         let chat = ChatMessage {
//             date: "test_date".to_string(),
//             user: get_her_pk(),
//             msg: expected_msg.clone(),
//         };

//         let send_msg_params = SendMsgParams {
//             ch_id: String::from(DUMMY_CHANNEL_ID_3),
//             msg: serde_json::to_string(&chat).unwrap(),
//         };

//         let args = serde_json::to_vec(&send_msg_params).unwrap();

//         let req = CtrRequest {
//             req_type: String::from("send_msg"),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let storage = mock_storage(&dummy_messeges);

//         (req, storage)
//     };

//     {
//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         let ctr_fn = ContractFn::Execute(request, storage);

//         let receipt = vm
//             .invoke(&ctr_wasm, ctr_fn)
//             .expect("State should be obtained");

//         let updated_storage = receipt.updated_storage.unwrap();

//         println!(
//             "updated_storage: {:#?}",
//             String::from_utf8(updated_storage.clone())
//         );

//         let storage: Storage = serde_json::from_slice(&updated_storage).unwrap();

//         let envelope_storage: EnvelopeStorage = serde_json::from_slice(&storage).unwrap();

//         let chats = envelope_storage.chats.get(DUMMY_CHANNEL_ID_3).unwrap();

//         let msg = chats.get(0).unwrap();

//         println!("expected msg: {:?}", expected_msg);

//         println!("updated msg: {:?}", msg);
//     };
// }

// #[tokio::test(flavor = "multi_thread")]
// async fn test_messenger_open_channel_me_and_you() {
//     EnvelopeTestUtils::init_test_log();

//     let vm = SakVM::init().expect("VM should be initiated");

//     let credential = CredentialProfile::test_1();

//     let test_dir = {
//         let tempdir = std::env::temp_dir()
//             .join("saksaha_test")
//             .join(credential.public_key_str);

//         std::fs::create_dir_all(&tempdir).unwrap();
//         tempdir
//     };

//     let mrs_path = { test_dir.join("mrs") };

//     let mrs = SakMRS::init(mrs_path).await.unwrap();

//     // let store_accessor = {
//     //     let a = StoreAccessor::new(mrs);
//     //     Arc::new(a)
//     // };

//     let my_pk = "my_pk".to_string();
//     let your_pk = "your_pk".to_string();

//     let mut open_ch_reqs = HashMap::new();

//     let mut chats = HashMap::<ChannelId, Vec<EncryptedChatMessage>>::new();

//     let chat_msg = ChatMessage {
//         date: "test_date".to_string(),
//         user: your_pk.clone(),
//         msg: "hello".to_string(),
//     };

//     chats.insert(
//         DUMMY_CHANNEL_ID_1.to_string(),
//         vec![serde_json::to_string(&chat_msg).unwrap()],
//     );

//     let envelope_storage = EnvelopeStorage {
//         open_ch_reqs,
//         chats,
//     };

//     let storage = serde_json::to_vec(&envelope_storage).unwrap();

//     let Channel {
//         ch_id,
//         eph_key,
//         // sig,
//         initiator_pk,
//         participants,
//     } = make_mock_open_ch();

//     let receipt_1 = {
//         let open_ch_params = OpenChParams {
//             dst_pk: my_pk.clone(),
//             open_ch: Channel {
//                 ch_id: ch_id.clone(),
//                 eph_key: eph_key.clone(),
//                 // sig: sig.clone(),
//                 initiator_pk: initiator_pk.clone(),
//                 participants: participants.clone(),
//             },
//         };

//         let args = serde_json::to_vec(&open_ch_params).unwrap();

//         let request = CtrRequest {
//             req_type: OPEN_CH.to_string(),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         let ctr_fn = ContractFn::Execute(request, storage.clone());

//         vm.invoke(&ctr_wasm, ctr_fn).unwrap()
//     };

//     let receipt_2 = {
//         let open_ch_params = OpenChParams {
//             dst_pk: your_pk.clone(),
//             open_ch: Channel {
//                 ch_id,
//                 eph_key,
//                 // sig,
//                 initiator_pk,
//                 participants,
//             },
//         };

//         let args = serde_json::to_vec(&open_ch_params).unwrap();

//         let request = CtrRequest {
//             req_type: OPEN_CH.to_string(),
//             args,
//             ctr_call_type: CtrCallType::Execute,
//         };

//         let ctr_wasm = ENVELOPE_CONTRACT.to_vec();
//         let ctr_fn = ContractFn::Execute(request, storage);

//         vm.invoke(&ctr_wasm, ctr_fn).unwrap()
//     };

//     {
//         let updated_storage_1 = receipt_1
//             .updated_storage
//             .ok_or("State needs to be updated, ")
//             .unwrap();

//         let storage_1: EnvelopeStorage = serde_json::from_slice(&updated_storage_1).unwrap();

//         let updated_storage_2 = receipt_2
//             .updated_storage
//             .ok_or("State needs to be updated, ")
//             .unwrap();

//         let storage_2: EnvelopeStorage = serde_json::from_slice(&updated_storage_2).unwrap();

//         println!("updated channel_1 id: {:?}", storage_1);
//         println!("updated channel_2 id: {:?}", storage_2);
//         let open_ch_reqs_1 = storage_1.open_ch_reqs.get(&my_pk).unwrap().get(0).unwrap();

//         let open_ch_reqs_2 = storage_2
//             .open_ch_reqs
//             .get(&your_pk)
//             .unwrap()
//             .get(0)
//             .unwrap();

//         assert_eq!(open_ch_reqs_1.ch_id, open_ch_reqs_2.ch_id);
//     }
// }
