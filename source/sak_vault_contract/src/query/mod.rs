use sak_contract_std::{ContractError, RequestArgs, Storage};

use crate::{GetValueParams, VaultStorage, VaultValue};

pub fn get_value(storage: Storage, args: RequestArgs) -> Result<Vec<u8>, ContractError> {
    let vault_storage: VaultStorage = match serde_json::from_slice(&storage) {
        Ok(s) => s,
        Err(err) => {
            return Err(format!(
                "Could not parse storage into envelope_storage, err: {}",
                err,
            )
            .into())
        }
    };

    let get_value_params: GetValueParams = match serde_json::from_slice(&args) {
        Ok(p) => p,
        Err(err) => return Err(format!("Could not parse `get_value_params`, err: {}", err).into()),
    };

    let value: VaultValue = match vault_storage.vault.get(&get_value_params.vault_key) {
        Some(v) => v.to_owned(),
        None => {
            return Err(format!(
                "Cannot find value from key: {:?}",
                get_value_params.vault_key,
            )
            .into());
        }
    };

    let res = match serde_json::to_vec(&value) {
        Ok(r) => r,
        Err(err) => return Err(format!("Could not serialize the value, err: {:?}", err).into()),
    };

    Ok(res)
}
