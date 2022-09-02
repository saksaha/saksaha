mod io;

use sak_kv_db::{BoundColumnFamily, ColumnFamilyDescriptor, Options, DB};
use std::sync::Arc;

pub(crate) struct Raw {
    pub db: DB,
}

impl Raw {
    pub(crate) fn make_cf_handle<'a>(
        &self,
        db: &'a DB,
        col_name: &'static str,
    ) -> Result<Arc<BoundColumnFamily<'a>>, String> {
        let cf_handle = match db.cf_handle(col_name) {
            Some(h) => h,
            None => {
                return Err(
                    format!("Fail to open ledger colums {}", col_name,),
                );
            }
        };

        Ok(cf_handle)
    }
}
