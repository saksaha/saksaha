use crate::{GetChListParams, GetMsgParams, OpenChParams, SendMsgParams};
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, CtrRequest, InvokeResult, RequestArgs, Storage,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use type_extension::U8Arr32;

pub mod request_type {
    pub const OPEN_CH: &'static str = "open_ch";
    pub const SEND_MSG: &'static str = "send_msg";
    pub const GET_CH_LIST: &'static str = "get_ch_list";
    pub const GET_MSG: &'static str = "get_msgs";
}

pub type PublicKey = String;
pub type ChannelId = String;
pub type Date = String;

pub const STORAGE_CAP: usize = 100;

pub struct OpenChReq {}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnvelopeStorage {
    pub open_ch_reqs: HashMap<PublicKey, Vec<Channel>>,
    pub chats: HashMap<ChannelId, Vec<ChatMessage>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub date: Date,
    pub user: PublicKey,
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChannelList {
    pub channels: Vec<Channel>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Channel {
    pub ch_id: String,
    pub eph_key: String,
    pub sig: String,
}

impl Channel {
    pub fn new(
        ch_id: String,
        eph_key: String,
        sig: String,
        key: U8Arr32,
    ) -> Result<Channel, ContractError> {
        let ch_id_enc = {
            let ch_id_enc = sak_crypto::aes_encrypt(&key, &ch_id.as_bytes())?;

            serde_json::to_string(&ch_id_enc)?
        };

        let sig_enc = {
            let sig_enc = sak_crypto::aes_encrypt(&key, &sig.as_bytes())?;

            serde_json::to_string(&sig_enc)?
        };

        let open_ch = Channel {
            ch_id: ch_id_enc,
            eph_key,
            sig: sig_enc,
        };

        Ok(open_ch)
    }

    pub fn default() -> Channel {
        Channel {
            ch_id: String::default(),
            eph_key: String::default(),
            sig: String::default(),
        }
    }
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
    request: CtrRequest,
    storage: Storage,
) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        request_type::GET_MSG => {
            return handle_get_msgs(storage, request.args);
        }
        request_type::GET_CH_LIST => {
            return handle_get_ch_list(storage, request.args);
        }
        _ => {
            return Err(
                format!("Wrong request type has been found in query").into()
            );
        }
    }
}

define_execute!();
pub fn execute2(
    request: CtrRequest,
    storage: &mut Storage,
) -> Result<InvokeResult, ContractError> {
    match request.req_type.as_ref() {
        request_type::OPEN_CH => {
            return handle_open_channel(storage, request.args);
        }
        request_type::SEND_MSG => {
            return handle_send_msg(storage, request.args);
        }
        _ => {
            return Err(format!(
                "Wrong request type has been found in execution"
            )
            .into());
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
            return Err(format!("err: {:?}", err).into());
        }
    };

    Ok(ret)
}

fn handle_open_channel(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    let mut evl_storage: EnvelopeStorage = match serde_json::from_slice(storage)
    {
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
        Err(err) => {
            return Err(
                format!("Could not parse open ch params, err: {}", err).into()
            )
        }
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
        Err(err) => {
            return Err(format!(
                "Cannot serialize envelope storage, err: {}",
                err
            )
            .into())
        }
    };

    Ok(vec![])
}

fn handle_send_msg(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<Vec<u8>, ContractError> {
    let mut evl_storage: EnvelopeStorage =
        match serde_json::from_slice(&storage) {
            Ok(e) => e,
            Err(err) => {
                return Err(format!(
                    "Failed to restore evl_storage, err: {:?}",
                    err
                )
                .into())
            }
        };

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

    if !evl_storage.chats.contains_key(&ch_id) {
        evl_storage.chats.insert(ch_id.clone(), vec![]);
    }

    let chats = evl_storage
        .chats
        .get_mut(&ch_id)
        .ok_or(format!("Channel is not initialied, ch_id: {}", ch_id))?;

    chats.push(send_msg_params.chat);

    *storage = serde_json::to_vec(&evl_storage)?;

    Ok(vec![])
}
