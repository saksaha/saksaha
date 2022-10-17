use sak_contract_std::{ContractError, RequestArgs, Storage};

use crate::{ContractCtx, GetValueParams, Key, StoreKey, StoreStorage, StoreValue};

// pub fn get_value(storage: Storage, args: RequestArgs) -> Result<Vec<u8>, ContractError> {
//     let store_storage: StoreStorage = match serde_json::from_slice(&storage) {
//         Ok(s) => s,
//         Err(err) => {
//             return Err(format!(
//                 "Could not parse storage into envelope_storage, err: {}",
//                 err,
//             )
//             .into())
//         }
//     };

//     let get_value_params: GetValueParams = match serde_json::from_slice(&args) {
//         Ok(p) => p,
//         Err(err) => return Err(format!("Could not parse `get_value_params`, err: {}", err).into()),
//     };

//     let store_key = get_value_params.store_key;

//     //
//     // key parsing logic should be implemented
//     //
//     //   1. mrs://{key}
//     //   2. ledger://{key}
//     //

//     let value: StoreValue = match store.store.get(&store_key) {
//         Some(v) => v.to_owned(),
//         None => {
//             return Err(format!("Could not find value stored with key: {:?}", store_key).into());
//         }
//     };

//     let res = match serde_json::to_vec(&value) {
//         Ok(r) => r,
//         Err(err) => return Err(format!("Could not serialize the value, err: {:?}", err).into()),
//     };

//     Ok(res)
// }

pub fn get_value(
    _ctx: ContractCtx,
    storage: Storage,
    args: RequestArgs,
) -> Result<Vec<u8>, ContractError> {
    let get_value_params: GetValueParams = serde_json::from_slice(&args)?;

    let store_key = get_value_params.store_key;

    let store_storage: StoreStorage = serde_json::from_slice(&storage)?;

    let key: Key = match store_key {
        StoreKey::MRS(key) => {
            // get_mrs_data
            // return key;

            key
        }
        StoreKey::LEDGER(key) => key,
    };

    let value: StoreValue = store_storage
        .store
        .get(&key)
        .ok_or(format!("Could not find value stored with key: {:?}", key))?
        .to_owned();

    let res = serde_json::to_vec(&value)?;

    Ok(res)
}
