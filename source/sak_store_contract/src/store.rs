use std::collections::HashMap;

use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, ContractError, CtrRequest,
    InvokeResult, Storage,
};

use crate::{
    execute::put_value,
    query::get_value,
    request_type::{GET_VALUE, PUT_VALUE},
    StoreStorage,
};

contract_bootstrap!();

#[link(wasm_import_module = "host")]
extern "C" {
    fn hello(param1: i32, param2: i32) -> i32;

    fn HOST__get_mrs_data(param1: *mut u8, param2: i32) -> i32;

    fn get_latest_len(p1: i32, p2: i32) -> i32;
}

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    let store_storage = StoreStorage {
        store: HashMap::new(),
    };

    let v = serde_json::to_vec(&store_storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: Storage,
) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        GET_VALUE => {
            return get_value(ctx, storage, request.args);
        }
        _ => {
            return Err(format!("Wrong request type has been found in query").into());
        }
    }
}

define_execute!();
pub fn execute2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: &mut Storage,
) -> Result<InvokeResult, ContractError> {
    match request.req_type.as_ref() {
        PUT_VALUE => {
            // ctx.put_mrs_data(arg)
            return put_value(ctx, storage, request.args);
        }
        // PUT_KEY_SPEC => {
        //     return put_key_spec(storage, request.args);
        // }
        _ => {
            return Err(format!("Wrong request type has been found in execution").into());
        }
    }
}
