// use crate::{cfs, CtrStateUpdate, LedgerDBSchema, MerkleUpdate};
use super::EnvelopeDBSchema;
use crate::db::cfs;
use crate::EnvelopeError;

impl EnvelopeDBSchema {
    pub async fn get_my_sk(
        &self,
        user_id: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SK)?;
        match self.db.get_cf(&cf, user_id)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_my_pk(
        &self,
        my_sk: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_PK)?;
        match self.db.get_cf(&cf, my_sk)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_my_sig(
        &self,
        my_sk: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SIG)?;
        match self.db.get_cf(&cf, my_sk)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_her_pk(
        &self,
        ch_id: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::HER_PK)?;
        match self.db.get_cf(&cf, ch_id)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_aes_key(
        &self,
        ch_id: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::AES_KEY)?;
        match self.db.get_cf(&cf, ch_id)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }
}
