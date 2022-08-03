use super::DBSchema;
use crate::db::cfs;
use crate::WalletError;
use sak_kv_db::WriteBatch;

impl DBSchema {
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

    pub(crate) async fn put_ch_data(
        &self,
        ch_id: &String,
        her_pk: &String,
        aes_key: &[u8; 32],
    ) -> Result<(), WalletError> {
        let mut batch = WriteBatch::default();

        let aes_key_str = serde_json::to_string(&aes_key)?;

        self.batch_put_her_pk(&mut batch, ch_id, her_pk)?;
        self.batch_put_aes_key(&mut batch, ch_id, &aes_key_str)?;

        self.db.write(batch)?;

        Ok(())
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

    pub(crate) fn batch_put_her_pk(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        ch_id: &String,
        her_pk: &String,
    ) -> Result<(), WalletError> {
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
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::AES_KEY)?;

        batch.put_cf(&cf, ch_id, aes_key);

        Ok(())
    }
}
