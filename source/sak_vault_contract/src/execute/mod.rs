use sak_contract_std::{ContractError, InvokeResult, RequestArgs, Storage};

use crate::{PutValueParams, VaultStorage};

pub fn put_value(storage: &mut Storage, args: RequestArgs) -> Result<InvokeResult, ContractError> {
    let mut vault_storage: VaultStorage = match serde_json::from_slice(storage) {
        Ok(s) => s,
        Err(err) => {
            return Err(format!(
                "Could not parse storage into envelope_storage, err: {}",
                err,
            )
            .into())
        }
    };

    let put_value_params: PutValueParams = match serde_json::from_slice(&args) {
        Ok(p) => p,
        Err(err) => return Err(format!("Could not parse `put_value_params`, err: {}", err).into()),
    };

    let vault_key = put_value_params.vault_key;
    let vault_value = put_value_params.vault_value;

    match vault_storage.vault.get_mut(&vault_key) {
        Some(_) => {
            return Err(format!("The channel is already opened").into());
        }
        None => {
            vault_storage.vault.insert(vault_key, vault_value);
        }
    };

    *storage = match serde_json::to_vec(&vault_storage) {
        Ok(s) => s,
        Err(err) => return Err(format!("Cannot serialize envelope storage, err: {}", err).into()),
    };

    Ok(vec![])
}

pub fn put_key_spec(
    storage: &mut Storage,
    args: RequestArgs,
) -> Result<InvokeResult, ContractError> {
    Ok(vec![])
}
