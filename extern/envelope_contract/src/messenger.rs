use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, Request, RequestArgs, Storage,
};
use std::collections::HashMap;

pub const ARG_CH_ID: &str = "ch_id";
pub const ARG_DST_PK: &str = "dst_pk";
pub const ARG_SERIALIZED_INPUT: &str = "serialized_input";
pub const STORAGE_CAP: usize = 100;

contract_bootstrap!();

define_init!();
pub fn init2() -> Storage {
    let storage_init = Storage::with_capacity(STORAGE_CAP);

    return storage_init;
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
        "open_channel" => {
            return handle_open_channel(storage, request.args);
        }
        "send_msg" => {
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
    let channel_id = match args.get(ARG_CH_ID) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("Args should contain a channel_id").into(),
            ));
        }
    };

    let msgs_serialized = match storage.get(channel_id) {
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
    let dst_pk = match args.get(ARG_DST_PK) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("Args should contain a channel_id").into(),
            ));
        }
    };

    let mut ch_list = vec![];

    match storage.get(dst_pk) {
        Some(o) => {
            let open_ch_data: Vec<String> =
                match serde_json::from_str(&o.as_str()) {
                    Ok(vs) => vs,
                    Err(err) => {
                        return Err(ContractError::new(
                            format!("err: {:?}", err).into(),
                        ));
                    }
                };

            for data in open_ch_data {
                let [_a, ch_id, _c]: [String; 3] =
                    match serde_json::from_str(&data) {
                        Ok(a) => a,
                        Err(err) => {
                            return Err(ContractError::new(
                                format!("err: {:?}", err).into(),
                            ));
                        }
                    };
                ch_list.push(ch_id);
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
    let dst_pk = match args.get(ARG_DST_PK) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("args should contain the her_pk").into(),
            ));
        }
    };

    let input_serialized = match args.get(ARG_SERIALIZED_INPUT) {
        Some(v) => v,
        None => {
            return Err(ContractError::new(
                format!("args should contain the input_serialized").into(),
            ));
        }
    };

    let (ch_id, open_ch_empty) = {
        let ret: Vec<String> = match serde_json::from_str(&input_serialized) {
            Ok(vs) => vs,
            Err(err) => {
                return Err(ContractError::new(
                    format!("err: {:?}", err).into(),
                ));
            }
        };
        (ret[1].clone(), ret[3].clone())
    };

    match storage.get_mut(&ch_id) {
        Some(_) => {
            return Err(ContractError::new(
                format!(
                    "The channel is already opened with the channel_id, {ch_id}"
                )
                .into(),
            ));
        }
        None => {}
    };

    match storage.get_mut(dst_pk) {
        Some(o) => {
            let mut open_ch_data: Vec<String> =
                match serde_json::from_str(&o.as_str()) {
                    Ok(vs) => vs,
                    Err(err) => {
                        return Err(ContractError::new(
                            format!("err: {:?}", err).into(),
                        ));
                    }
                };
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
