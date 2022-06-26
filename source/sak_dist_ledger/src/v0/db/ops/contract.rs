use crate::{LedgerDB, LedgerError};
use sak_kv_db::WriteBatch;

impl LedgerDB {
    pub(crate) async fn get_ctr_data(
        &self,
        ctr_addr: &String,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let db = &self.kv_db.db_instance;

        // let cf_handle = match db.cf_handle(columns::DATA) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger colums {}",
        //             columns::CTR_STATE
        //         ));
        //     }
        // };

        // let key = ctr_addr;

        // let value = match db.get_cf(cf_handle, &key) {
        //     Ok(val) => match val {
        //         Some(v) => v,
        //         None => return Ok(None),
        //     },
        //     Err(err) => {
        //         return Err(format!(
        //             "Fail to get value from ledger columns, column: {}, \
        //             err: {}",
        //             columns::CTR_STATE,
        //             err,
        //         ));
        //     }
        // };

        let data = self
            .schema
            .get_data(db, ctr_addr)?
            .ok_or("ctr data does not exist")?;

        Ok(Some(data))
    }

    pub(crate) async fn get_contract_state(
        &self,
        ctr_addr: &String,
        field_name: &String,
    ) -> Result<Option<String>, String> {
        let db = &self.kv_db.db_instance;

        // let cf_handle = match db.cf_handle(columns::CTR_STATE) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger colums {}",
        //             columns::CTR_STATE
        //         ));
        //     }
        // };

        // let key = format!("{}:{}", contract_addr, field_name);

        // let value = match db.get_cf(cf_handle, &key) {
        //     Ok(val) => match val {
        //         Some(v) => match std::str::from_utf8(&v) {
        //             Ok(vs) => vs.to_string(),
        //             Err(err) => {
        //                 return Err(format!(
        //                     "Invalid utf8 given, err: {}",
        //                     err,
        //                 ));
        //             }
        //         },
        //         None => {
        //             return Err(format!(
        //                 "No matched value with key in {}, key: {}",
        //                 columns::CTR_STATE,
        //                 &key,
        //             ));
        //         }
        //     },
        //     Err(err) => {
        //         return Err(format!(
        //             "Fail to get value from ledger columns, column: {}, \
        //             err: {}",
        //             columns::CTR_STATE,
        //             err,
        //         ));
        //     }
        // };

        let ctr_state = self
            .schema
            .get_ctr_state(db, ctr_addr, field_name)?
            .ok_or("ctr state does not exist")?;

        Ok(Some(ctr_state))
    }

    pub(crate) async fn set_contract_state(
        &self,
        contract_addr: &String,
        field_name: &String,
        field_value: &String,
    ) -> Result<String, String> {
        let db = &self.ledger_db.db_instance;

        let mut batch = WriteBatch::default();

        let cf_handle = match db.cf_handle(columns::CTR_STATE) {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Fail to open ledger columns {}",
                    columns::CTR_STATE
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
