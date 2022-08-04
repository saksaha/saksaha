use std::collections::HashMap;

use crate::{GetChListParams, GetMsgParams, OpenChParams};
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, Request, RequestArgs, Storage,
};
use serde::{Deserialize, Serialize};

pub mod request_type {
    pub const OPEN_CH: &'static str = "open_ch";
    pub const SEND_MSG: &'static str = "send_msg";
}

pub const ARG_CH_ID: &str = "ch_id";

pub const ARG_DST_PK: &str = "dst_pk";

pub const ARG_SERIALIZED_INPUT: &str = "serialized_input";

pub const STORAGE_CAP: usize = 100;

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgStorage {
    channels: HashMap<String, Vec<OpenCh>>,
    chats: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenCh {
    ch_id: String,
    eph_key: String,
    sig: String,
}

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    // let storage_init = Storage::with_capacity(STORAGE_CAP);
    let storage = MsgStorage {
        channels: HashMap::new(),
        chats: HashMap::new(),
    };

    let v = serde_json::to_vec(&storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    request: Request,
    storage: Storage,
) -> Result<String, ContractError> {
    match request.req_type.as_ref() {
        "get_msgs" => {
            return handle_get_msgs(storage, request.args);
        }
        "get_ch_list" => {
            return handle_get_ch_list(storage, request.args);
        }
        _ => {
            return Err(ContractError::new(
                format!("Wrong request type has been found").into(),
            ));
        }
    }
}

define_execute!();
pub fn execute2(
    storage: &mut Storage,
    request: Request,
) -> Result<(), ContractError> {
    match request.req_type.as_ref() {
        request_type::OPEN_CH => {
            return handle_open_channel(storage, request.args);
        }
        request_type::SEND_MSG => {
            return handle_send_msg(storage, request.args);
        }
        _ => {
            return Err(ContractError::new(
                format!("Wrong request type has been found").into(),
            ));
        }
    }
}

fn handle_get_msgs(
    storage: Storage,
    args: RequestArgs,
) -> Result<String, ContractError> {
    let msg_storage: MsgStorage = serde_json::from_slice(&storage)?;

    let get_msg_params: GetMsgParams = serde_json::from_slice(&args)?;

    // let channel_id = match args.get(ARG_CH_ID) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("Args should contain a channel_id").into(),
    //         ));
    //     }
    // };

    let msgs_serialized = match msg_storage.chats.get(&get_msg_params.ch_id) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("Chat should be obtained").into(),
            ));
        }
    };

    Ok(msgs_serialized.clone())
}

fn handle_get_ch_list(
    storage: Storage,
    args: RequestArgs,
) -> Result<String, ContractError> {
    let msg_storage: MsgStorage = serde_json::from_slice(&storage)?;

    let get_ch_list_params: GetChListParams = serde_json::from_slice(&args)?;

    // let dst_pk = match args.get(&get_ch_list_params.dst_pk) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("Args should contain a channel_id").into(),
    //         ));
    //     }
    // };

    let mut ch_list = vec![];

    match msg_storage.channels.get(&get_ch_list_params.dst_pk) {
        Some(open_channels) => {
            // let open_ch_data: Vec<String> =
            //     match serde_json::from_str(&o.as_str()) {
            //         Ok(vs) => vs,
            //         Err(err) => {
            //             return Err(ContractError::new(
            //                 format!("err: {:?}", err).into(),
            //             ));
            //         }
            //     };

            for open_ch in open_channels {
                // let [_a, ch_id, _c]: [String; 3] =
                //     match serde_json::from_str(&data) {
                //         Ok(a) => a,
                //         Err(err) => {
                //             return Err(ContractError::new(
                //                 format!("err: {:?}", err).into(),
                //             ));
                //         }
                //     };

                ch_list.push(open_ch);
            }
        }
        None => {}
    }

    let ch_list_serialized = match serde_json::to_string(&ch_list) {
        Ok(s) => s,
        Err(err) => {
            return Err(ContractError::new(format!("err: {:?}", err).into()));
        }
    };

    Ok(ch_list_serialized.clone())
}

fn handle_open_channel(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let msg_storage: MsgStorage = serde_json::from_slice(&storage)?;

    let open_ch_params: OpenChParams = serde_json::from_slice(&args)?;

    // let dst_pk = match args.get(ARG_DST_PK) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("args should contain the her_pk").into(),
    //         ));
    //     }
    // };

    // (ch_id, eph_key, sig)
    // let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("args should contain the input_serialized").into(),
    //         ));
    //     }
    // };

    let (ch_id, open_ch_empty) = {
        let ret: Vec<String> =
            match serde_json::from_slice(&open_ch_params.input_serialized) {
                Ok(vs) => vs,
                Err(err) => {
                    return Err(ContractError::new(
                        format!("err: {:?}", err).into(),
                    ));
                }
            };

        (ret[1].clone(), ret[3].clone())
    };

    match msg_storage.chats.get_mut(&ch_id) {
        Some(_) => {
            return Err(ContractError::new(
                format!("The channel is already opened").into(),
            ));
        }
        None => {}
    };

    match msg_storage.channels.get_mut(&open_ch_params.dst_pk) {
        Some(open_channels) => {
            // let mut open_ch_data: Vec<String> =
            //     match serde_json::from_str(&o.as_str()) {
            //         Ok(vs) => vs,
            //         Err(err) => {
            //             return Err(ContractError::new(
            //                 format!("err: {:?}", err).into(),
            //             ));
            //         }
            //     };

            // open_ch_data.push(input_serialized.clone());

            // let input_serialized_new =
            //     match serde_json::to_string(&open_ch_data) {
            //         Ok(s) => s,
            //         Err(err) => {
            //             return Err(ContractError::new(
            //                 format!("err: {:?}", err).into(),
            //             ));
            //         }
            //     };

            storage.insert(dst_pk.clone(), input_serialized_new);
        }
        None => {
            let mut open_ch_data = vec![];

            open_ch_data.push(input_serialized.clone());

            let input_serialized_new =
                match serde_json::to_string(&open_ch_data) {
                    Ok(s) => s,
                    Err(err) => {
                        return Err(ContractError::new(
                            format!("err: {:?}", err).into(),
                        ));
                    }
                };

            storage.insert(dst_pk.clone(), input_serialized_new.clone());
        }
    };

    storage.insert(ch_id.clone(), open_ch_empty);

    Ok(())
}

fn handle_send_msg(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let channel_id = match args.get(ARG_CH_ID) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("args should contain the channel_id").into(),
            ));
        }
    };

    let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("args should contain the msg").into(),
            ));
        }
    };

    storage.insert(channel_id.clone(), input_serialized.clone());

    Ok(())
}
