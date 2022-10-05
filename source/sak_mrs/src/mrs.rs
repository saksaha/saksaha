use sak_contract_std::{
    contract_bootstrap, define_execute, define_init, define_query,
    ContractError, CtrRequest, InvokeResult, RequestArgs, Storage,
};
use sak_crypto::SakKey;
use sak_crypto::ToEncodedPoint;
use std::collections::HashMap;

use crate::{request_type::RESERVE, MutableRecordStorage, Slot};

const SLOT_CAPACITY: usize = 64;

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
        _ => Err(("Wrong request type has been found in query").into()),
    }
}

define_execute!();
pub fn execute2(
    request: CtrRequest,
    storage: &mut Storage,
) -> Result<InvokeResult, ContractError> {
    match request.req_type.as_ref() {
        RESERVE => reserve_slot(storage, request.args),
        _ => Err(("Wrong request type has been found in execution").into()),
    }
}

fn reserve_slot(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    let mut mrs: MutableRecordStorage = serde_json::from_slice(storage)?;

    let (sk, pk) = SakKey::generate();
    let public_key =
        sak_crypto::encode_hex(&pk.to_encoded_point(false).to_bytes());

    mrs.slots.insert(public_key, Slot::new());

    *storage = serde_json::to_vec(&mrs)?;

    Ok(vec![])
}

fn get_empty_slot_idx() {}
