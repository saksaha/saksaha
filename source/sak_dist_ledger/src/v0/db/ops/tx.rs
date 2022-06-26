use crate::{LedgerDB, LedgerError};
use sak_kv_db::{
    DBRawIteratorWithThreadMode, DBWithThreadMode, KeyValueDatabase,
    SingleThreaded, WriteBatch,
};
use sak_types::Tx;

impl LedgerDB {
    pub(crate) async fn write_tx(
        &self,
        tx: &Tx,
    ) -> Result<String, LedgerError> {
        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        let tx_hash = tx.get_hash();

        // let cf_handle = match db.cf_handle(columns::CREATED_AT) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger columns {}",
        //             columns::CREATED_AT
        //         ))
        //     }
        // };
        // batch.put_cf(cf_handle, tx_hash, tx.get_created_at());

        self.schema.batch_put_created_at(
            db,
            &mut batch,
            tx_hash,
            tx.get_created_at(),
        )?;

        // let data_cf = match db.cf_handle(columns::DATA) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger columns {}",
        //             columns::DATA
        //         ))
        //     }
        // };
        // batch.put_cf(data_cf, tx_hash, tx.get_data());

        self.schema
            .batch_put_data(db, &mut batch, tx_hash, tx.get_data())?;

        // let cf_handle = match db.cf_handle(columns::PI) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger columns {}",
        //             columns::PI
        //         ))
        //     }
        // };
        // batch.put_cf(cf_handle, tx_hash, tx.get_pi());

        self.schema
            .batch_put_pi(db, &mut batch, tx_hash, tx.get_pi())?;

        // let cf_handle = match db.cf_handle(columns::SIG_VEC) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger columns {}",
        //             columns::SIG_VEC
        //         ))
        //     }
        // };
        // batch.put_cf(cf_handle, tx_hash, tx.get_signature());

        self.schema.batch_put_author_sig(
            db,
            &mut batch,
            tx_hash,
            tx.get_author_sig(),
        )?;

        // let cf_handle = match db.cf_handle(columns::CONTRACT_ADDR) {
        //     Some(h) => h,
        //     None => {
        //         return Err(format!(
        //             "Fail to open ledger columns {}",
        //             columns::CONTRACT_ADDR,
        //         ))
        //     }
        // };
        // batch.put_cf(cf_handle, tx_hash, tx.get_contract_addr());

        self.schema.batch_put_ctr_addr(
            db,
            &mut batch,
            tx_hash,
            tx.get_ctr_addr(),
        )?;

        // K: contract_addr => V: contract_bytecode (data)
        // batch.put_cf(data_cf, tx.get_contract_addr(), tx.get_data());

        self.schema.batch_put_tx_hash(
            db,
            &mut batch,
            tx.get_ctr_addr(),
            tx_hash,
        )?;

        db.write(batch)?;

        return Ok(tx_hash.clone());
    }

    pub(crate) async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        let db = &self.kv_db.db_instance;

        // let mut tx_value_result = vec![
        //     String::from(""),
        //     String::from(""),
        //     String::from(""),
        //     String::from(""),
        //     String::from(""),
        // ];

        // let tx_values_col = vec![
        //     columns::CREATED_AT,
        //     columns::DATA,
        //     columns::SIG_VEC,
        //     columns::PI,
        //     columns::CONTRACT_ADDR,
        // ];

        // let tx_values_it_map = tx_values_col.iter().map(|cf_name| cf_name);

        // for (idx, cfn) in tx_values_it_map.enumerate() {
        //     let cf_handle = match db.cf_handle(cfn) {
        //         Some(h) => h,
        //         None => {
        //             return Err(format!("Fail to open ledger columns {}", cfn));
        //         }
        //     };

        //     tx_value_result[idx] = match db.get_cf(cf_handle, &tx_hash) {
        //         Ok(val) => match val {
        //             Some(v) => match std::str::from_utf8(&v) {
        //                 Ok(vs) => vs.to_string(),
        //                 Err(err) => {
        //                     return Err(format!(
        //                         "Invalid utf8 given, err: {}",
        //                         err,
        //                     ));
        //                 }
        //             },
        //             None => {
        //                 return Err(format!(
        //                     "No matched value with tx_hash in {}, {}",
        //                     cfn, &tx_hash,
        //                 ));
        //             }
        //         },
        //         Err(err) => {
        //             return Err(format!(
        //                 "Fail to get value from ledger columns, column: {}, \
        //                 err: {}",
        //                 cfn, err,
        //             ));
        //         }
        //     };
        // }

        let created_at = self
            .schema
            .get_created_at(db, tx_hash)?
            .ok_or("created_at does not exist")?;

        let data = self
            .schema
            .get_data(db, tx_hash)?
            .ok_or("data does not exist")?;

        let author_sig = self
            .schema
            .get_author_sig(db, tx_hash)?
            .ok_or("author_sig does not exist")?;

        let pi = self
            .schema
            .get_pi(db, tx_hash)?
            .ok_or("pi does not exist")?;

        let ctr_addr = self
            .schema
            .get_ctr_addr(db, tx_hash)?
            .ok_or("ctr_addr does not exist")?;

        let tx = Tx::new(created_at, data, author_sig, pi, Some(ctr_addr));

        Ok(Some(tx))
    }
}

pub mod testing {
    use super::*;

    impl LedgerDB {
        // pub fn iter(
        //     &self,
        // ) -> DBRawIteratorWithThreadMode<DBWithThreadMode<SingleThreaded>>
        // {
        //     let db = &self.kv_db.db_instance;

        //     let iter =
        //         db.raw_iterator_cf(db.cf_handle(columns::CREATED_AT).unwrap());

        //     iter
        // }

        pub fn delete_tx(&self, tx_hash: &String) -> Result<(), LedgerError> {
            let db = &self.kv_db.db_instance;

            // let created_at_cf = match db.cf_handle(columns::CREATED_AT) {
            //     Some(h) => h,
            //     None => {
            //         return Err(format!(
            //             "Fail to open ledger columns `crated_at`"
            //         ))
            //     }
            // };

            // match db.delete_cf(created_at_cf, key) {
            //     Ok(_) => (),
            //     Err(err) => {
            //         return Err(format!(
            //             "Error deleting column family created_at, err: {}",
            //             err,
            //         ));
            //     }
            // }

            let mut batch = WriteBatch::default();

            self.schema
                .batch_delete_created_at(db, &mut batch, tx_hash)?;

            // let data_cf = match db.cf_handle(columns::DATA) {
            //     Some(h) => h,
            //     None => {
            //         return Err(format!("Fail to open ledger columns `DATA`"))
            //     }
            // };
            // match db.delete_cf(data_cf, key) {
            //     Ok(_) => (),
            //     Err(err) => {
            //         return Err(format!(
            //             "Error deleting column family data_cf, err: {}",
            //             err,
            //         ));
            //     }
            // }

            self.schema.batch_delete_data(db, &mut batch, tx_hash)?;

            // let pi_cf = match db.cf_handle(columns::PI) {
            //     Some(h) => h,
            //     None => {
            //         return Err(format!("Fail to open ledger columns `PI`"))
            //     }
            // };
            // match db.delete_cf(pi_cf, key) {
            //     Ok(_) => (),
            //     Err(err) => {
            //         return Err(format!(
            //             "Error deleting column family pi, err: {}",
            //             err,
            //         ));
            //     }
            // }

            self.schema.batch_delete_pi(db, &mut batch, tx_hash)?;

            // let sig_vec_cf = match db.cf_handle(columns::SIG_VEC) {
            //     Some(h) => h,
            //     None => {
            //         return Err(format!(
            //             "Fail to open ledger columns `SIG_VEC`"
            //         ))
            //     }
            // };

            // match db.delete_cf(sig_vec_cf, key) {
            //     Ok(_) => (),
            //     Err(err) => {
            //         return Err(format!(
            //             "Error deleting column family sig_vec, err: {}",
            //             err,
            //         ));
            //     }
            // }

            self.schema
                .batch_delete_author_sig(db, &mut batch, tx_hash)?;

            Ok(())
        }
    }
}
