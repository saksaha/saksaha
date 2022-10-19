use crate::{request_type::RESERVE, MutableRecordStorage, ReserveSlotParams, Slot};
use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query, ContractError, CtrRequest,
    InvokeResult, RequestArgs, Storage,
};

const SLOT_CAPACITY: usize = 64;

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
    // storage: Storage,
) -> Result<Vec<u8>, ContractError> {
    // let storage = vec![];

    match request.req_type.as_ref() {
        "unimplemented" => {
            unimplemented!()
        }
        _ => Err(("Wrong request type has been found in query").into()),
    }
}

define_execute!();
pub fn execute2(request: CtrRequest, storage: &mut Storage) -> Result<InvokeResult, ContractError> {
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

fn get_empty_slot_idx() {}
