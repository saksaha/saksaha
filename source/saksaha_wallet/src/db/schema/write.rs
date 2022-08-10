use super::WalletDBSchema;
use crate::db::cfs;
use crate::WalletError;
use sak_kv_db::WriteBatch;

impl WalletDBSchema {
    pub(crate) async fn put_coin_data(
        &self,
        cm: &String,
        rho: &String,
        r: &String,
        s: &String,
        v: &String,
        a_pk: &String,
        a_sk: &String,
        user_id: &String,
        status: &String,
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

    pub(crate) async fn put_user_data(
        &self,
        my_id: &String,
        sk: &String,
        pk: &String,
        sig: &String,
    ) -> Result<String, WalletError> {
        let mut batch = WriteBatch::default();

        self.batch_put_my_sk(&mut batch, my_id, sk)?;
        self.batch_put_my_pk(&mut batch, sk, pk)?;
        self.batch_put_my_sig(&mut batch, sk, sig)?;

        self.db.write(batch)?;

        Ok(sk.to_string())
    }

    pub(crate) fn batch_put_my_sk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        user_id: &String,
        sk: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SK)?;
        batch.put_cf(&cf, user_id, sk);

        Ok(())
    }

    pub(crate) fn batch_put_my_pk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        sk: &String,
        pk: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_PK)?;

        batch.put_cf(&cf, sk, pk);

        Ok(())
    }

    pub(crate) fn batch_put_my_sig(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        sk: &String,
        sig: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SIG)?;

        batch.put_cf(&cf, sk, sig);

        Ok(())
    }
}
