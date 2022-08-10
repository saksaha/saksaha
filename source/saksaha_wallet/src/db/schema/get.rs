use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;

impl WalletDBSchema {
    pub async fn get_status(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_user_id(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_rho(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::RHO)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_r(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::R)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_s(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_v(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_a_pk(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_PK)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_a_sk(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_SK)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_cm_idx(
        &self,
        cm: &String,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub async fn get_cm(
        &self,
        cm_idx: &u128,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let cm_idx = cm_idx.to_be_bytes();

        match self.db.get_cf(&cf, cm_idx)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_latest_cm_idx(
        &self,
        // db: &DB,
    ) -> Result<Option<u128>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let mut iter = self.db.iterator_cf(&cf, sak_kv_db::IteratorMode::End);

        let (cm_idx_bytes, _hash) = match iter.next() {
            Some(a) => a,
            None => return Ok(None),
        };

        let cm_idx = sak_kv_db::convert_u8_slice_into_u128(&cm_idx_bytes)?;

        Ok(Some(cm_idx))
    }

    pub async fn get_my_sk(
        &self,
        user_id: &String,
    ) -> Result<Option<String>, WalletError> {
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
}
