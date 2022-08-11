use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;
use sak_crypto::Scalar;
use sak_proofs::{OldCoin, CM_TREE_DEPTH};

impl WalletDBSchema {
    pub fn get_coin(&self, cm: &String) -> Result<OldCoin, WalletError> {
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

        let addr_pk = Scalar::from(addr_pk.parse::<u64>()?);
        let addr_sk = Scalar::from(addr_sk.parse::<u64>()?);
        let rho = Scalar::from(rho.parse::<u64>()?);
        let r = Scalar::from(r.parse::<u64>()?);
        let s = Scalar::from(s.parse::<u64>()?);
        let v = Scalar::from(v.parse::<u64>()?);
        let cm = Scalar::from(cm.parse::<u64>()?);

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

    pub fn get_status(
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

    pub fn get_user_id(
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

    pub fn get_rho(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_r(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_s(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_v(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_a_pk(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_a_sk(&self, cm: &String) -> Result<Option<String>, WalletError> {
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

    pub fn get_cm_idx(
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

    pub fn get_cm(&self, cm_idx: &u128) -> Result<Option<String>, WalletError> {
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

    pub fn get_my_sk(
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
