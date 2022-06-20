use sak_kv_db::WriteBatch;
use sak_types::ContractState;

use crate::{columns, Database};

impl Database {
    pub(crate) async fn get_contract_state_value(
        &self,
        // key
        contract_addr_and_field: &String,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let cf_handle = match db.cf_handle(columns::CONTRACT_STATE) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger colums {}",
                    columns::VALIDATOR_SIG
                ));
            }
        };

        let value = match db.get_cf(cf_handle, &contract_addr_and_field) {
            Ok(val) => match val {
                Some(v) => match std::str::from_utf8(&v) {
                    Ok(vs) => vs.to_string(),
                    Err(err) => {
                        return Err(format!(
                            "Invalid utf8 given, err: {}",
                            err,
                        ));
                    }
                },
                None => {
                    return Err(format!(
                        "No matched value with tx_hash in {}, {}",
                        columns::CONTRACT_STATE,
                        &contract_addr_and_field,
                    ));
                }
            },
            Err(err) => {
                return Err(format!(
                    "Fail to get value from ledger columns, column: {}, \
                    err: {}",
                    columns::CONTRACT_STATE,
                    err,
                ));
            }
        };

        Ok(value)
    }

    pub(crate) async fn set_contract_state(
        &self,
        state: &ContractState,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let mut batch = WriteBatch::default();

        let cf_handle = match db.cf_handle(columns::CONTRACT_STATE) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CREATED_AT
                ))
            }
        };

        let contract_addr = state.get_contract_addr();

        for iter in state //
            .get_field_name()
            .iter()
            .zip(state.get_field_value())
        {
            let (field, value) = iter;

            let key = format!("{}:{}", contract_addr, (*field).clone());

            batch.put_cf(cf_handle, key, (*value).clone());
        }

        match db.write(batch) {
            Ok(_) => return Ok("".to_string().clone()),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }
}
