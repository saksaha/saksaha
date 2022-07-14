use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, Request,
    Storage,
};
use std::collections::HashMap;

type ExecuteArgs = HashMap<String, String>;

// open_channel const types
pub(crate) const ARG_CH_ID: &str = "ch_id";
pub(crate) const ARG_CIPHER_TEXT: &str = "cipher_text";
pub(crate) const ARG_EPH_PK: &str = "eph_pk";
pub(crate) const ARG_SRC_PK: &str = "src_pk";
pub(crate) const ARG_DST_PK: &str = "dst_pk";
pub(crate) const ARG_SERIALIZED_INPUT: &str = "serialized_input";
pub(crate) const DUMMY_CHANNEL_ID_1: &str = "ch_12";

const ARG_MESSAGE: &str = "message";
const STORAGE_CAP: usize = 100;

contract_bootstrap!();

define_init!();
pub fn init2() -> Storage {
    let mut storage_init = Storage::with_capacity(STORAGE_CAP);

    let dummy_chat = match serde_json::to_string(&vec![
        String::from("Hi, there"),
        String::from("This is a secret message"),
    ]) {
        Ok(s) => s,
        Err(err) => panic!("Cannot serialize messages, err: {}", err),
    };

    storage_init.insert(String::from(DUMMY_CHANNEL_ID_1), dummy_chat);

    return storage_init;
}

define_query!();
pub fn query2(request: Request, storage: Storage) -> String {
    match request.req_type.as_ref() {
        "get_msgs" => {
            return handle_get_msgs(storage, request.arg);
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };
}

define_execute!();
pub fn execute2(storage: &mut Storage, request: Request) {
    match request.req_type.as_ref() {
        "open_channel" => {
            handle_open_channel(storage, request.arg);
        }
        "send_msg" => {
            handle_send_msg(storage, request.arg);
        }
        _ => {
            panic!("Wrong request type has been found");
        }
    };
}

fn handle_get_msgs(storage: Storage, args: ExecuteArgs) -> String {
    let channel_id = match args.get(ARG_CH_ID) {
        Some(v) => v,
        None => {
            panic!("Args should contain a channel_id");
        }
    };

    let msgs_serialized = match storage.get(channel_id) {
        Some(v) => v,
        None => {
            panic!("Chat should be obtained");
        }
    };

    msgs_serialized.clone()
    // let msgs_ptr = msgs.as_mut_ptr();
    // let msgs_len = msgs.len();

    // std::mem::forget(msgs);

    // (msgs_ptr, msgs_len as i32)
}

fn handle_open_channel(storage: &mut Storage, args: ExecuteArgs) {
    let dst_pk = match args.get(ARG_DST_PK) {
        Some(v) => v,
        None => {
            panic!("args should contain the her_pk");
        }
    };

    let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
        Some(v) => v,
        None => {
            panic!("args should contain the input_serialized");
        }
    };

    let [_eph_pk_str, ch_id, _open_ch_src, open_ch_empty]: [String; 4] =
        serde_json::from_str(&input_serialized.as_str()).unwrap();

    match storage.get_mut(&ch_id) {
        Some(_) => {
            panic!(
                "The channel is already opened with the channel_id, {ch_id}"
            );
        }
        None => {}
    };

    storage.insert(dst_pk.clone(), input_serialized.clone()); // put in the open_channel storage
    storage.insert(ch_id.clone(), open_ch_empty); // put in the chats storage
}

fn handle_send_msg(storage: &mut Storage, args: ExecuteArgs) {
    let channel_id = match args.get(ARG_CH_ID) {
        Some(v) => v,
        None => {
            panic!("args should contain the channel_id");
        }
    };

    let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
        Some(v) => v,
        None => {
            panic!("args should contain the msg");
        }
    };

    storage.remove(channel_id);

    storage.insert(channel_id.clone(), input_serialized.clone());
}
