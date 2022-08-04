use crate::{GetChListParams, GetMsgParams, OpenChParams, SendMsgParams};
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, Request, RequestArgs, Storage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod request_type {
    pub const OPEN_CH: &'static str = "open_ch";
    pub const SEND_MSG: &'static str = "send_msg";
}

pub type PublicKey = String;
pub type ChannelId = String;

pub const STORAGE_CAP: usize = 100;

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvelopeStorage {
    pub open_ch_reqs: HashMap<PublicKey, Vec<OpenCh>>,
    pub chats: HashMap<ChannelId, Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OpenCh {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    let evl_storage = EnvelopeStorage {
        open_ch_reqs: HashMap::new(),
        chats: HashMap::new(),
    };

    let v = serde_json::to_vec(&evl_storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    request: Request,
    storage: Storage,
) -> Result<Vec<u8>, ContractError> {
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
) -> Result<Vec<u8>, ContractError> {
    let evl_storage: EnvelopeStorage = serde_json::from_slice(&storage)?;

    let get_msg_params: GetMsgParams = serde_json::from_slice(&args)?;

    // let channel_id = match args.get(ARG_CH_ID) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("Args should contain a channel_id").into(),
    //         ));
    //     }
    // };

    // let msgs_serialized = match msg_storage.chats.get(&get_msg_params.ch_id) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("Chat should be obtained").into(),
    //         ));
    //     }
    // };

    let ch_id = get_msg_params.ch_id;

    let chats = evl_storage
        .chats
        .get(&ch_id)
        .ok_or(format!("Chat is not initialized, ch_id: {}", &ch_id))?;

    let ret = serde_json::to_vec(chats)?;

    Ok(ret)
}

fn handle_get_ch_list(
    storage: Storage,
    args: RequestArgs,
) -> Result<Vec<u8>, ContractError> {
    let evl_storage: EnvelopeStorage = serde_json::from_slice(&storage)?;

    let get_ch_list_params: GetChListParams = serde_json::from_slice(&args)?;

    let mut ch_list = vec![];

    match evl_storage.open_ch_reqs.get(&get_ch_list_params.dst_pk) {
        Some(open_channels) => {
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

    let ret = match serde_json::to_vec(&ch_list) {
        Ok(s) => s,
        Err(err) => {
            return Err(ContractError::new(format!("err: {:?}", err).into()));
        }
    };

    Ok(ret)
}

fn handle_open_channel(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let mut evl_storage: EnvelopeStorage = serde_json::from_slice(&storage)?;

    let open_ch_params: OpenChParams = serde_json::from_slice(&args)?;

    let dst_pk = open_ch_params.dst_pk;
    let open_ch = open_ch_params.open_ch;

    // (ch_id, eph_key, sig)
    // let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("args should contain the input_serialized").into(),
    //         ));
    //     }
    // };

    // let (ch_id, open_ch_empty) = {
    //     let ret: Vec<String> =
    //         match serde_json::from_slice(&open_ch_params.input_serialized) {
    //             Ok(vs) => vs,
    //             Err(err) => {
    //                 return Err(ContractError::new(
    //                     format!("err: {:?}", err).into(),
    //                 ));
    //             }
    //         };

    //     (ret[1].clone(), ret[3].clone())
    // };

    match evl_storage.chats.get_mut(&open_ch.ch_id) {
        Some(_) => {
            return Err(ContractError::new(
                format!("The channel is already opened").into(),
            ));
        }
        None => {}
    };

    // let open_ch = OpenCh {
    //     ch_id: open_.ch_id,
    //     eph_key: open_ch_params.eph_pk,
    //     sig: open_ch_params.sig,
    // };

    match evl_storage.open_ch_reqs.get_mut(&dst_pk) {
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

            open_channels.push(open_ch);

            // serde_json::to_vec

            // msg_storage.insert(dst_pk.clone(), open_channels);
        }
        None => {
            // let mut open_ch_data = vec![];

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

            evl_storage.open_ch_reqs.insert(dst_pk, vec![open_ch]);

            // storage.insert(dst_pk.clone(), input_serialized_new.clone());
        }
    };

    // storage.insert(ch_id.clone(), open_ch_empty);

    *storage = serde_json::to_vec(&evl_storage)?;

    Ok(())
}

fn handle_send_msg(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<(), ContractError> {
    let mut evl_storage: EnvelopeStorage = serde_json::from_slice(&storage)?;

    let send_msg_params: SendMsgParams = serde_json::from_slice(&args)?;

    // let channel_id = match args.get(ARG_CH_ID) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("args should contain the channel_id").into(),
    //         ));
    //     }
    // };

    // let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
    //     Some(v) => v,
    //     None => {
    //         return Err(ContractError::new(
    //             format!("args should contain the msg").into(),
    //         ));
    //     }
    // };

    // storage.insert(channel_id.clone(), input_serialized.clone());

    let ch_id = send_msg_params.ch_id;

    let chats = evl_storage
        .chats
        .get_mut(&ch_id)
        .ok_or(format!("Channel is not initialied, ch_id: {}", ch_id))?;

    chats.push(send_msg_params.msg);

    *storage = serde_json::to_vec(&evl_storage)?;

    Ok(())
}
