use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;
use crate::types::Status;
use sak_kv_db::WriteBatch;

impl WalletDBSchema {
    // getter
    pub async fn get_status(
        &self,
        cm: &String,
    ) -> Result<Option<Status>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;
        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                // let str = String::from_utf8(v)?;
                let status: Status = Status::from_u8(v)?;

                return Ok(Some(status));
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

    // setter
    pub(crate) async fn put_coin(
        &self,
        cm: &String,
        rho: &String,
        r: &String,
        s: &String,
        v: &String,
        a_pk: &String,
        a_sk: &String,
        user_id: &String,
        status: &Status,
        cm_idx: &u128,
    ) -> Result<(), WalletError> {
        let mut batch = WriteBatch::default();

        self.batch_put_rho(&mut batch, cm, rho)?;
        self.batch_put_r(&mut batch, cm, r)?;
        self.batch_put_s(&mut batch, cm, s)?;
        self.batch_put_v(&mut batch, cm, v)?;
        self.batch_put_a_pk(&mut batch, cm, a_pk)?;
        self.batch_put_a_sk(&mut batch, cm, a_sk)?;
        self.batch_put_user_id(&mut batch, cm, user_id)?;
        self.batch_put_status(&mut batch, cm, status)?;
        self.batch_put_cm_idx(&mut batch, cm, cm_idx)?;

        self.batch_put_cm_by_cm_idx(&mut batch, cm_idx, cm)?;

        self.db.write(batch)?;

        Ok(())
    }

    pub(crate) async fn put_status(
        &self,
        cm: &String,
        status: &Status,
    ) -> Result<(), WalletError> {
        let mut batch = WriteBatch::default();

        self.batch_put_status(&mut batch, cm, status)?;

        self.db.write(batch)?;

        Ok(())
    }

    pub(crate) fn batch_put_rho(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        rho: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::RHO)?;

        batch.put_cf(&cf, cm, rho);

        Ok(())
    }

    pub(crate) fn batch_put_r(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        r: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::R)?;

        batch.put_cf(&cf, cm, r);

        Ok(())
    }

    pub(crate) fn batch_put_s(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        s: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;

        batch.put_cf(&cf, cm, s);

        Ok(())
    }

    pub(crate) fn batch_put_v(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        v: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;

        batch.put_cf(&cf, cm, v);

        Ok(())
    }

    pub(crate) fn batch_put_a_pk(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        a_pk: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_PK)?;

        batch.put_cf(&cf, cm, a_pk);

        Ok(())
    }

    pub(crate) fn batch_put_a_sk(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        a_sk: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_SK)?;

        batch.put_cf(&cf, cm, a_sk);

        Ok(())
    }

    pub(crate) fn batch_put_user_id(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        user_id: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;

        batch.put_cf(&cf, cm, user_id);

        Ok(())
    }

    pub(crate) fn batch_put_status(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        status: &Status,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;

        batch.put_cf(&cf, cm, status);

        Ok(())
    }

    pub(crate) fn batch_put_cm_idx(
        &self,
        batch: &mut WriteBatch,
        cm: &String,
        cm_idx: &u128,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

        let cm_idx = cm_idx.to_be_bytes();

        batch.put_cf(&cf, cm, cm_idx);

        Ok(())
    }

    pub(crate) fn batch_put_cm_by_cm_idx(
        &self,
        batch: &mut WriteBatch,
        cm_idx: &u128,
        cm: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let cm_idx = cm_idx.to_be_bytes();

        batch.put_cf(&cf, cm_idx, cm);

        Ok(())
    }
}
