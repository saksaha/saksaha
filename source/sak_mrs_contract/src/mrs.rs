use crate::{request_type::RESERVE, MutableRecordStorage, ReserveSlotParams, Slot};
use sak_contract_derive::MRSStore;
use sak_contract_std::{
    contract_bootstrap, ContractError, CtrRequest, InvokeResult, RequestArgs, Storage,
};

const SLOT_CAPACITY: usize = 64;

contract_bootstrap!();

#[derive(MRSStore, Debug)]
pub struct St {}

pub fn init() -> Result<Storage, ContractError> {
    let evl_storage = MutableRecordStorage {
        slots: vec![Slot::default()],
    };

    let v = serde_json::to_vec(&evl_storage)?;

    Ok(v)
}

pub fn query(ctx: ContractCtx<St>, request: CtrRequest) -> Result<Vec<u8>, ContractError> {
    // let storage = vec![];

    match request.req_type.as_ref() {
        "unimplemented" => {
            unimplemented!()
        }
        _ => Err(("Wrong request type has been found in query").into()),
    }
}

pub fn update(ctx: ContractCtx<St>, request: CtrRequest) -> Result<InvokeResult, ContractError> {
    let mut storage = vec![];
    match request.req_type.as_ref() {
        RESERVE => reserve_slot(&mut storage, request.args),
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
