use crate::{request_type::RESERVE, MutableRecordStorage, ReserveSlotParams, Slot};
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, ContractError, CtrRequest,
    InvokeResult, RequestArgs, Storage,
};

const SLOT_CAPACITY: usize = 16;

#[link(wasm_import_module = "host")]
extern "C" {
    fn hello(param1: i32, param2: i32) -> i32;

    fn HOST__get_mrs_data(param1: *mut u8, param2: i32) -> i32;
    fn HOST__put_mrs_data(param1: *mut u8, param2: i32) -> i32;

    fn get_latest_len(p1: i32, p2: i32) -> i32;
}

contract_bootstrap!();

define_init!();
pub fn init2() -> Result<Storage, ContractError> {
    let evl_storage = MutableRecordStorage {
        slots: vec![Slot::default()],
    };

    let v = serde_json::to_vec(&evl_storage)?;

    Ok(v)
}

define_query!();
pub fn query2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: Storage,
) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        "unimplemented" => {
            unimplemented!()
        }
        _ => Err(("Wrong request type has been found in query").into()),
    }
}

define_execute!();
pub fn execute2(
    ctx: ContractCtx,
    request: CtrRequest,
    storage: &mut Storage,
) -> Result<InvokeResult, ContractError> {
    match request.req_type.as_ref() {
        RESERVE => reserve_slot(storage, request.args),
        _ => Err(("Wrong request type has been found in execution").into()),
    }
}

fn reserve_slot(storage: &mut Storage, args: RequestArgs) -> Result<InvokeResult, ContractError> {
    let mut mrs: MutableRecordStorage = serde_json::from_slice(storage)?;
    let reserve_slot_params: ReserveSlotParams = serde_json::from_slice(&args)?;

    *storage = serde_json::to_vec(&mrs)?;

    Ok(vec![])
}

// fn update(storage: &mut Storage, args: RequestArgs) -> Result<InvokeResult, ContractError> {
//     let mut mrs: MutableRecordStorage = serde_json::from_slice(storage)?;
//     let reserve_slot_params: ReserveSlotParams = serde_json::from_slice(&args)?;

//     *storage = serde_json::to_vec(&mrs)?;

//     Ok(vec![])
// }
