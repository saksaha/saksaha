use sak_contract_std::{ContractError, InvokeResult, RequestArgs, Storage};

use crate::{ContractCtx, Key, PutValueParams, StoreKey, StoreStorage};

// pub fn put_value(storage: &mut Storage, args: RequestArgs) -> Result<InvokeResult, ContractError> {
//     let mut store_storage: StoreStorage = match serde_json::from_slice(storage) {
//         Ok(s) => s,
//         Err(err) => {
//             return Err(format!(
//                 "Could not parse storage into envelope_storage, err: {}",
//                 err,
//             )
//             .into())
//         }
//     };

//     let put_value_params: PutValueParams = match serde_json::from_slice(&args) {
//         Ok(p) => p,
//         Err(err) => return Err(format!("Could not parse `put_value_params`, err: {}", err).into()),
//     };

//     let store_key = put_value_params.store_key;
//     let store_value = put_value_params.store_value;

//     match store_storage.store.get_mut(&store_key) {
//         Some(_) => {
//             return Err(format!("The channel is already opened").into());
//         }
//         None => {
//             store_storage.store.insert(store_key, store_value);
//         }
//     };

//     *storage = match serde_json::to_vec(&store_storage) {
//         Ok(s) => s,
//         Err(err) => return Err(format!("Cannot serialize envelope storage, err: {}", err).into()),
//     };

//     Ok(vec![])
// }

pub fn put_value(
    _ctx: ContractCtx,
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    let mut store_storage: StoreStorage = serde_json::from_slice(storage)?;

    let put_value_params: PutValueParams = serde_json::from_slice(&args)?;

    let store_key = put_value_params.store_key;
    let store_value = put_value_params.store_value;

    let key: Key = match store_key {
        StoreKey::MRS(key) => {
            // put_mrs_data
            // return key;

            key
        }
        StoreKey::LEDGER(key) => key,
    };

    match store_storage.store.get_mut(&key) {
        Some(_) => {
            return Err(format!("store with key: {:?} will be overwritten", key).into());
        }
        None => {
            store_storage.store.insert(key, store_value);
        }
    };

    *storage = serde_json::to_vec(&store_storage)?;

    Ok(vec![])
}
