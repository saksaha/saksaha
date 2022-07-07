use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, Request,
    Storage,
};
use std::collections::HashMap;

type ExecuteArgs = HashMap<String, String>;

const ARG_CHANNEL_ID: &str = "channel_id";
const ARG_MESSAGE: &str = "message";
const ARG_HER_PK: &str = "her_pk";
const DUMMY_CHANNEL_ID_1: &str = "dummy_channel_1";
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
    let channel_id = match args.get(ARG_CHANNEL_ID) {
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
    let her_pk = match args.get(ARG_HER_PK) {
        Some(v) => v,
        None => {
            panic!("args should contain the her_pk");
        }
    };

    let channel_id = match args.get(ARG_CHANNEL_ID) {
        Some(v) => v,
        None => {
            panic!("args should contain the channel_id");
        }
    };

    match storage.get_mut(channel_id) {
        Some(_) => {
            panic!("The channel is already opened with the channel_id, {channel_id}");
        }
        None => {}
    };

    storage.insert(her_pk.clone(), channel_id.clone());
}

fn handle_send_msg(storage: &mut Storage, args: ExecuteArgs) {
    let channel_id = match args.get(ARG_CHANNEL_ID) {
        Some(v) => v,
        None => {
            panic!("args should contain the channel_id");
        }
    };

    let msg_new = match args.get(ARG_MESSAGE) {
        Some(v) => v,
        None => {
            panic!("args should contain the msg");
        }
    };

    let msgs_serialized = match storage.get_mut(channel_id) {
        Some(v) => v,
        None => {
            panic!("storage should contain the channel_id");
        }
    };

    let mut msgs_vec: Vec<String> =
        match serde_json::from_str(msgs_serialized.as_str()) {
            Ok(v) => v,
            Err(err) => {
                panic!("should be contained in vector, err: {}", err);
            }
        };

    msgs_vec.push(msg_new.clone());

    let chat_new = serde_json::to_string(&msgs_vec).unwrap();

    storage.remove(channel_id);

    storage.insert(channel_id.clone(), chat_new);
}
