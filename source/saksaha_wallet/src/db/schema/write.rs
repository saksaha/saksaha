use super::DBSchema;
use crate::db::cfs;
use crate::WalletError;
use sak_kv_db::WriteBatch;

impl DBSchema {
    pub(crate) async fn put_coin_data(
        &self,
        // my_id: &String,
        // sk: &String,
        // pk: &String,
        // sig: &String,
        cm: &String,
        rho: &String,
        r: &String,
        s: &String,
        v: &String,
        a_pk: &String,
        a_sk: &String,
        user_id: &String,
        status: &String,
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

        self.db.write(batch)?;

        Ok(())
    }

    pub(crate) async fn put_status(
        &self,
        // ch_id: &String,
        cm: &String,
        status: &String,
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
        status: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;

        batch.put_cf(&cf, cm, status);

        Ok(())
    }
}
