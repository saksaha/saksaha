use crate::{v0::db::CFSenum, v0::db::MRSDB, MRSError};
use sak_crypto::Proof;
use sak_crypto::{Bls12, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_kv_db::DB;
use std::convert::TryInto;

impl MRSDB {
    pub fn get_dummy(&self, key: &String) -> Result<Option<String>, MRSError> {
        let cf = self.make_cf_handle(&self.db, CFSenum::MrsValue.as_str())?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn batch_put_dummy(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &String,
        value: &String,
    ) -> Result<(), MRSError> {
        let cf = self.make_cf_handle(&self.db, CFSenum::MrsValue.as_str())?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
