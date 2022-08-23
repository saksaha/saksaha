use super::EnvelopeDBSchema;
use crate::db::cfs;
use crate::EnvelopeError;
use envelope_contract::EncryptedEphSecret;

impl EnvelopeDBSchema {
    pub async fn get_my_sk_by_acc_addr(
        &self,
        acc_addr: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SK)?;
        match self.db.get_cf(&cf, acc_addr)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_my_pk_by_sk(
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

    pub async fn get_my_sig_by_sk(
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

    pub async fn get_her_pk_by_ch_id(
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

    pub async fn get_aes_key_by_ch_id(
        &self,
        ch_id: &String,
    ) -> Result<Option<EncryptedEphSecret>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::SHARED_SECRET_KEY)?;
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

    pub async fn get_my_acc_addr_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Option<String>, EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::ACC_ADDR)?;
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
}
