use crate::{columns, Database};
use sak_kv_db::WriteBatch;

impl Database {
    pub(crate) async fn get_contract_state(
        &self,
        contract_addr: &String,
        field_name: &String,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let cf_handle = match db.cf_handle(columns::CONTRACT_STATE) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger colums {}",
                    columns::CONTRACT_STATE
                ));
            }
        };

        let key = format!("{}:{}", contract_addr, field_name);

        let value = match db.get_cf(cf_handle, &key) {
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
                        "No matched value with key in {}, key: {}",
                        columns::CONTRACT_STATE,
                        &key,
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
        contract_addr: &String,
        field_name: &String,
        field_value: &String,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let mut batch = WriteBatch::default();

        let cf_handle = match db.cf_handle(columns::CONTRACT_STATE) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CONTRACT_STATE
                ))
            }
        };

        let key = format!("{}:{}", contract_addr, field_name);

        batch.put_cf(cf_handle, key, field_value);

        match db.write(batch) {
            Ok(_) => return Ok("".to_string().clone()),
            Err(err) => {
                return Err(format!("Fail to write on ledger db, err: {}", err))
            }
        }
    }
}
