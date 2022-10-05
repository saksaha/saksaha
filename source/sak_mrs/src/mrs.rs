use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, CtrRequest, InvokeResult, RequestArgs, Storage,
};
use std::collections::HashMap;

use crate::{request_type::RESERVE, MutableRecordStorage};

pub const STORAGE_CAP: usize = 100;

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    let evl_storage = MutableRecordStorage {
        slots: HashMap::new(),
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
        "unimplemented" => {
            unimplemented!()
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
        RESERVE => {
            return reserve_slot(storage, request.args);
        }
        _ => {
            return Err(format!(
                "Wrong request type has been found in execution"
            )
            .into());
        }
    }
}

fn reserve_slot(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    let mut mrs: MutableRecordStorage = serde_json::from_slice(storage)?;

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
