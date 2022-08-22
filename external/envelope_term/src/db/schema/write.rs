use super::EnvelopeDBSchema;
use crate::db::cfs;
use crate::EnvelopeError;
use sak_kv_db::WriteBatch;

impl EnvelopeDBSchema {
    pub(crate) async fn put_user_data(
        &self,
        sk: &String,
        pk: &String,
        sig: &String,
        acc_addr: &String,
    ) -> Result<String, EnvelopeError> {
        let mut batch = WriteBatch::default();

        self.batch_put_my_sk(&mut batch, acc_addr, sk)?;
        self.batch_put_my_pk(&mut batch, sk, pk)?;
        self.batch_put_my_sig(&mut batch, sk, sig)?;

        self.db.write(batch)?;

        Ok(sk.to_string())
    }

    pub(crate) async fn put_ch_shared_secret_key(
        &self,
        ch_id: &String,
        // her_pk: &String,
        aes_key: &String,
    ) -> Result<(), EnvelopeError> {
        let mut batch = WriteBatch::default();

        // let aes_key_str = serde_json::to_string(&aes_key)?;

        // self.batch_put_her_pk(&mut batch, ch_id, her_pk)?;
        self.batch_put_aes_key(&mut batch, ch_id, &aes_key)?;

        self.db.write(batch)?;

        Ok(())
    }

    pub(crate) fn batch_put_my_sk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        acc_addr: &String,
        sk: &String,
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SK)?;
        batch.put_cf(&cf, acc_addr, sk);

        Ok(())
    }

    pub(crate) fn batch_put_my_pk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        sk: &String,
        pk: &String,
    ) -> Result<(), EnvelopeError> {
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
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::MY_SIG)?;

        batch.put_cf(&cf, sk, sig);

        Ok(())
    }

    pub(crate) fn batch_put_her_pk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        ch_id: &String,
        her_pk: &String,
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::HER_PK)?;

        batch.put_cf(&cf, ch_id, her_pk);

        Ok(())
    }

    pub(crate) fn batch_put_aes_key(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        ch_id: &String,
        aes_key: &String,
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::AES_KEY)?;

        batch.put_cf(&cf, ch_id, aes_key);

        Ok(())
    }

    pub(crate) fn batch_put_ch_id(
        &self,
        batch: &mut WriteBatch,
        ch_idx: &u128,
        ch_id: &String,
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::CH_ID)?;

        let v = ch_idx.to_be_bytes();

        batch.put_cf(&cf, &v, ch_id);

        Ok(())
    }

    pub(crate) fn batch_put_my_acc_addr(
        &self,
        batch: &mut WriteBatch,
        user_id: &String,
        acc_addr: &String,
    ) -> Result<(), EnvelopeError> {
        let cf = self.make_cf_handle(&self.db, cfs::ACC_ADDR)?;
        batch.put_cf(&cf, user_id, acc_addr);

        Ok(())
    }
}
