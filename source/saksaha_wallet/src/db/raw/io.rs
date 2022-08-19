use super::Raw;
use crate::db::cfs;
use crate::WalletError;
use sak_crypto::{Scalar, ScalarExt};
use sak_kv_db::DBIteratorWithThreadMode;
use sak_kv_db::DBWithThreadMode;
use sak_kv_db::MultiThreaded;
use sak_kv_db::WriteBatch;
use sak_types::CoinIdx;
use sak_types::CoinStatus;

impl Raw {
    pub(crate) fn get_coin_iter(
        &self,
    ) -> Result<
        DBIteratorWithThreadMode<DBWithThreadMode<MultiThreaded>>,
        WalletError,
    > {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let iter = self.db.iterator_cf(&cf, sak_kv_db::IteratorMode::Start);

        Ok(iter)
    }

    pub(crate) fn get_latest_coin_idx(
        &self,
    ) -> Result<Option<CoinIdx>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let mut iter = self.db.iterator_cf(&cf, sak_kv_db::IteratorMode::End);

        match iter.next() {
            Some((c_idx, _cm)) => {
                let coin_idx =
                    type_extension::convert_u8_slice_into_u128(&c_idx)?;

                return Ok(Some(coin_idx));
            }
            None => return Ok(None),
        }
    }

    pub(crate) fn get_coin_idx(
        &self,
        cm: &Scalar,
    ) -> Result<Option<CoinIdx>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::COIN_IDX)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let coin_idx = type_extension::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(coin_idx));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_coin_status(
        &self,
        cm: &Scalar,
    ) -> Result<Option<CoinStatus>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::COIN_STATUS)?;

        let cm = cm.to_bytes();

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let status: CoinStatus = CoinStatus::from_u8(v)?;

                return Ok(Some(status));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_cm(
        &self,
        coin_idx: &CoinIdx,
    ) -> Result<Option<Scalar>, WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let coin_idx = coin_idx.to_be_bytes();

        match self.db.get_cf(&cf, coin_idx)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;
                let cm = ScalarExt::parse_arr(&arr)?;

                return Ok(Some(cm));
            }
            None => {
                return Ok(None);
            }
        };
    }

    // pub fn get_user_id(
    //     &self,
    //     cm: &Scalar,
    // ) -> Result<Option<String>, WalletError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::USER_ID)?;

    //     let cm = cm.to_bytes();

    //     match self.db.get_cf(&cf, cm)? {
    //         Some(v) => {
    //             let str = String::from_utf8(v)?;

    //             return Ok(Some(str));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     };
    // }

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

    pub(crate) fn batch_put_coin_status(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        status: &CoinStatus,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::COIN_STATUS)?;

        let cm = cm.to_bytes();

        batch.put_cf(&cf, cm, status);

        Ok(())
    }

    pub(crate) fn single_put_coin_status(
        &self,
        cm: &Scalar,
        status: &CoinStatus,
    ) -> Result<(), WalletError> {
        let mut batch = WriteBatch::default();

        let cf = self.make_cf_handle(&self.db, cfs::COIN_STATUS)?;

        let cm = cm.to_bytes();

        batch.put_cf(&cf, cm, status);

        Ok(())
    }

    pub(crate) fn batch_put_coin_idx(
        &self,
        batch: &mut WriteBatch,
        cm: &Scalar,
        coin_idx: &CoinIdx,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::COIN_IDX)?;

        let cm = cm.to_bytes();

        let coin_idx = coin_idx.to_be_bytes();

        batch.put_cf(&cf, cm, coin_idx);

        Ok(())
    }

    pub(crate) fn batch_put_cm(
        &self,
        batch: &mut WriteBatch,
        coin_idx: &CoinIdx,
        cm: &Scalar,
    ) -> Result<(), WalletError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        let cm = cm.to_bytes();

        let coin_idx = coin_idx.to_be_bytes();

        batch.put_cf(&cf, coin_idx, cm);

        Ok(())
    }
}
