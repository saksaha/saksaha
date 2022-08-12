use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;
use crate::types::Status;
use sak_crypto::{Scalar, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_proofs::{OldCoin, CM_TREE_DEPTH};
use sak_types::U8Arr32;

impl WalletDBSchema {
    pub fn get_coin(&self, cm: &U8Arr32) -> Result<OldCoin, WalletError> {
        let addr_pk = match self.get_a_pk(&cm)? {
            Some(p) => p,
            None => return Err(format!("Failed to get a_pk").into()),
        };

        let addr_sk = match self.get_a_sk(&cm)? {
            Some(s) => s,
            None => return Err(format!("Failed to get a_sk").into()),
        };

        let rho = match self.get_rho(&cm)? {
            Some(r) => r,
            None => return Err(format!("Failed to get rho").into()),
        };

        let r = match self.get_r(&cm)? {
            Some(r) => r,
            None => return Err(format!("Failed to get r").into()),
        };

        let s = match self.get_s(&cm)? {
            Some(s) => s,
            None => return Err(format!("Failed to get s").into()),
        };

        let v = match self.get_v(&cm)? {
            Some(v) => v,
            None => return Err(format!("Failed to get v").into()),
        };

        // let addr_pk = Scalar::from(addr_pk.parse::<u64>()?);
        let addr_pk = ScalarExt::parse_arr(&addr_pk)?;

        // let addr_sk = Scalar::from(addr_sk.parse::<u64>()?);
        let addr_sk = ScalarExt::parse_arr(&addr_sk)?;

        // let rho = Scalar::from(rho.parse::<u64>()?);
        let rho = ScalarExt::parse_arr(&rho)?;

        // let r = Scalar::from(r.parse::<u64>()?);
        let r = ScalarExt::parse_arr(&r)?;

        // let s = Scalar::from(s.parse::<u64>()?);
        let s = ScalarExt::parse_arr(&s)?;

        // let v = Scalar::from(v.parse::<u64>()?);
        let v = ScalarExt::parse_arr(&v)?;

        // let cm = Scalar::from(cm.parse::<u64>()?);
        let cm = ScalarExt::parse_arr(&cm)?;

        let old_coin = OldCoin {
            addr_pk: Some(addr_pk),
            addr_sk: Some(addr_sk),
            rho: Some(rho),
            r: Some(r),
            s: Some(s),
            v: Some(v),
            cm: Some(cm),
            auth_path: [None; CM_TREE_DEPTH as usize],
        };

        Ok(old_coin)
    }

    pub async fn get_status(
        &self,
        cm: &Scalar,
    ) -> Result<Option<Status>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let status: Status = Status::from_u8(v)?;

                return Ok(Some(status));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_user_id(
        &self,
        cm: &Scalar,
    ) -> Result<Option<String>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;

        let cm = cm.to_bytes();

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

    pub fn get_rho(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::RHO)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(rho) => {
                let val = ScalarExt::parse_vec(rho)?;
                // let str = String::from_utf8(v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_r(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::R)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(r) => {
                // let str = String::from_utf8(v)?;
                let val = ScalarExt::parse_vec(r)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_s(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(s) => {
                let val = ScalarExt::parse_vec(s)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_v(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let val = ScalarExt::parse_vec(v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_a_pk(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_PK)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let val = ScalarExt::parse_vec(v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub fn get_a_sk(&self, cm: &Scalar) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_SK)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let val = ScalarExt::parse_vec(v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        };
    }

    // pub fn get_cm(
    //     &self,
    //     cm_idx: &u128,
    // ) -> Result<Option<U8Arr32>, WalletError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM)?;

    //     // let cm_idx = cm_idx.to_be_bytes();

    //     match self.db.get_cf(&cf, cm_idx)? {
    //         Some(v) => {
    //             let str = String::from_utf8(v)?;

    //             return Ok(Some(str));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     };
    // }

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

    pub(crate) async fn put_coin(
        &self,
        cm: &Scalar,
        rho: &Scalar,
        r: &Scalar,
        s: &Scalar,
        v: &Scalar,
        a_pk: &Scalar,
        a_sk: &Scalar,
        user_id: &String,
        status: &Status,
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
        // self.batch_put_cm_idx(&mut batch, cm, cm_idx)?;

        // self.batch_put_cm_by_cm_idx(&mut batch, cm_idx, cm)?;

        self.db.write(batch)?;

        Ok(())
    }

    pub(crate) async fn put_status(
        &self,
        cm: &Scalar,
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
        cm: &Scalar,
        rho: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::RHO)?;

        let cm = cm.to_bytes();
        let rho = rho.to_bytes();

        batch.put_cf(&cf, cm, rho);

        Ok(())
    }

    pub(crate) fn batch_put_r(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        r: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::R)?;

        let cm = cm.to_bytes();
        let r = r.to_bytes();

        batch.put_cf(&cf, cm, r);

        Ok(())
    }

    pub(crate) fn batch_put_s(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        s: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;

        let cm = cm.to_bytes();
        let s = s.to_bytes();

        batch.put_cf(&cf, cm, s);

        Ok(())
    }

    pub(crate) fn batch_put_v(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        v: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;

        let cm = cm.to_bytes();
        let v = v.to_bytes();

        batch.put_cf(&cf, cm, v);

        Ok(())
    }

    pub(crate) fn batch_put_a_pk(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        a_pk: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_PK)?;

        let cm = cm.to_bytes();
        let a_pk = a_pk.to_bytes();

        batch.put_cf(&cf, cm, a_pk);

        Ok(())
    }

    pub(crate) fn batch_put_a_sk(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        a_sk: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::A_SK)?;

        let cm = cm.to_bytes();
        let a_sk = a_sk.to_bytes();

        batch.put_cf(&cf, cm, a_sk);

        Ok(())
    }

    pub(crate) fn batch_put_user_id(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        user_id: &String,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;

        let cm = cm.to_bytes();

        batch.put_cf(&cf, cm, user_id);

        Ok(())
    }

    pub(crate) fn batch_put_status(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        status: &Status,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::STATUS)?;

        let cm = cm.to_bytes();

        batch.put_cf(&cf, cm, status);

        Ok(())
    }
}
