use crate::{
    request_type::{GET_CH_LIST, GET_MSG, OPEN_CH, SEND_MSG},
    EnvelopeStorage, GetChListParams, GetMsgParams, OpenChParams, SendMsgParams,
};
use sak_contract_derive::{CtrStateStore, MRSStore};
use sak_contract_std::{
    contract_bootstrap, ContractError, CtrRequest, Dict, InvokeResult, List, RequestArgs, Storage,
};
use std::collections::HashMap;

pub const STORAGE_CAP: usize = 100;

pub struct OpenChReq {}

contract_bootstrap!();

#[derive(Debug, MRSStore)]
pub struct SomeMRSStorage {
    pub chats: List,
    pub channels: Dict,
}

pub fn init() -> Result<Storage, ContractError> {
    let evl_storage = EnvelopeStorage {
        open_ch_reqs: HashMap::new(),
        chats: HashMap::new(),
    };

    let v = serde_json::to_vec(&evl_storage)?;

    Ok(v)
}

pub fn query(ctx: ContractCtx, request: CtrRequest) -> Result<Vec<u8>, ContractError> {
    // let storage = vec![]; // soon will be removed

    unsafe {
        let param = "key".to_string();

        let a = ctx.mrs.chats.get(&"power".to_string());
        // let a = ctx.mrs.channels.get(&"!!!key!!!".to_string());

        // let data2 = ctx.get_mrs_data(&param); // consecutive call works, too

        return Ok(a);
    }

    // match request.req_type.as_ref() {
    //     GET_MSG => {
    //         return get_msgs(storage, request.args);
    //     }
    //     GET_CH_LIST => {
    //         return get_ch_list(storage, request.args);
    //     }
    //     _ => {
    //         return Err(format!("Wrong request type has been found in query").into());
    //     }
    // }
}

pub fn update(
    ctx: ContractCtx,
    request: CtrRequest,
    // storage: &mut Storage
) -> Result<InvokeResult, ContractError> {
    let mut storage = vec![];
    match request.req_type.as_ref() {
        OPEN_CH => {
            return handle_open_channel(&mut storage, request.args);
        }
        SEND_MSG => {
            return handle_send_msg(&mut storage, request.args);
        }
        _ => {
            return Err(format!("Wrong request type has been found in execution").into());
        }
    }
}

fn get_msgs(storage: Storage, args: RequestArgs) -> Result<Vec<u8>, ContractError> {
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

fn get_ch_list(storage: Storage, args: RequestArgs) -> Result<Vec<u8>, ContractError> {
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
            return Err(format!("err: {:?}", err).into());
        }
    };

    Ok(ret)
}

fn handle_open_channel(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    let mut evl_storage: EnvelopeStorage = match serde_json::from_slice(storage) {
        Ok(s) => s,
        Err(err) => {
            return Err(format!(
                "Could not parse storage into envelope_storage, err: {}",
                err,
            )
            .into())
        }
    };

    let open_ch_params: OpenChParams = match serde_json::from_slice(&args) {
        Ok(p) => p,
        Err(err) => return Err(format!("Could not parse open ch params, err: {}", err).into()),
    };

    let dst_pk = open_ch_params.dst_pk;
    let open_ch = open_ch_params.open_ch;

    match evl_storage.chats.get_mut(&open_ch.ch_id) {
        Some(_) => {
            return Err(format!("The channel is already opened").into());
        }
        None => {}
    };

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

    *storage = match serde_json::to_vec(&evl_storage) {
        Ok(s) => s,
        Err(err) => return Err(format!("Cannot serialize envelope storage, err: {}", err).into()),
    };

    Ok(vec![])
}

fn handle_send_msg(storage: &mut Storage, args: RequestArgs) -> Result<Vec<u8>, ContractError> {
    // let a: _MRS;
    let mut evl_storage: EnvelopeStorage = match serde_json::from_slice(&storage) {
        Ok(e) => e,
        Err(err) => return Err(format!("Failed to restore evl_storage, err: {:?}", err).into()),
    };

    let send_msg_params: SendMsgParams = serde_json::from_slice(&args)?;

    let ch_id = send_msg_params.ch_id;

    if !evl_storage.chats.contains_key(&ch_id) {
        evl_storage.chats.insert(ch_id.clone(), vec![]);
    }

    let chats = evl_storage
        .chats
        .get_mut(&ch_id)
        .ok_or(format!("Channel is not initialized, ch_id: {}", ch_id))?;

    chats.push(send_msg_params.msg);

    *storage = serde_json::to_vec(&evl_storage)?;

    // mrs
    // let arg = struct A {
    //     data_chunk: vec![],
    //     sig: "",
    //     slot_id: 00,
    //     ts: 0,
    //     old_ts,
    // };

    // ctx.put_mrs_data(arg_ptr, arg_len);

    Ok(vec![])
}
