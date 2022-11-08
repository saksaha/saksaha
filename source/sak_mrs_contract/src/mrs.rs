use crate::{
    request_type::{GET_SLOT, RESERVE},
    ReserveSlotParams, Slot,
};
use sak_contract_derive::{CtrStateStore, MRSStore};
use sak_contract_std::{
    saksaha_contract, ContractError, CtrRequest, InvokeResult, List, RequestArgs, Storage,
};

const SLOT_CAPACITY: usize = 64;

saksaha_contract!(0.0.1);

#[derive(MRSStore, Debug)]
pub struct SomeMRSMRSStore {}

#[derive(Debug, CtrStateStore)]
pub struct SomeMRSCtrState {
    pub slots: List<String>,
}

pub fn init(ctx: &mut ContractCtx) -> Result<Storage, ContractError> {
    let next_slot_number = ctx.ctr_state.slots.len() + 1;

    let new_slot = Slot::new(
        String::from("Initial public key"),
        String::from("Current Time"),
        next_slot_number,
    );

    ctx.ctr_state.slots.push(serde_json::to_vec(&new_slot)?);

    Ok(next_slot_number.to_be_bytes().to_vec())
}

pub fn execute(ctx: &ContractCtx, request: CtrRequest) -> Result<Vec<u8>, ContractError> {
    match request.req_type.as_ref() {
        GET_SLOT => get_slot(ctx),
        _ => Err(format!(
            "Wrong request type has been found in query : {:?}",
            request.req_type,
        )
        .into()),
    }
}

pub fn update(ctx: ContractCtx, request: CtrRequest) -> Result<InvokeResult, ContractError> {
    match request.req_type.as_ref() {
        RESERVE => rent(ctx, request.args),
        _ => Err(("Wrong request type has been found in execution").into()),
    }
}

fn rent(mut ctx: ContractCtx, args: RequestArgs) -> Result<InvokeResult, ContractError> {
    let rent_params: ReserveSlotParams = serde_json::from_slice(&args)?;

    let next_slot_number = ctx.ctr_state.slots.len() + 1;

    let new_slot = Slot::new(
        rent_params.public_key,
        String::from("Current Time"),
        next_slot_number,
    );

    ctx.ctr_state.slots.push(serde_json::to_vec(&new_slot)?);

    Ok(next_slot_number.to_be_bytes().to_vec())
}

fn get_slot(ctx: &ContractCtx) -> Result<InvokeResult, ContractError> {
    let last_idx = ctx.ctr_state.slots.len();

    let slot = ctx.ctr_state.slots.get(&last_idx.to_string());

    Ok(slot)
}
