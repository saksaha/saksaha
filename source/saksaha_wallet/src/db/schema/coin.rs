use super::WalletDBSchema;
use crate::app::WalletError;
use crate::db::cfs;
use sak_crypto::{Scalar, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_proofs::{OldCoin, CM_TREE_DEPTH};
use sak_types::CoinStatus;
use type_extension::U8Arr32;

impl WalletDBSchema {
    pub fn get_coin(&self, cm: &Scalar) -> Result<OldCoin, WalletError> {
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

        let old_coin = OldCoin {
            addr_pk: Some(addr_pk),
            addr_sk: Some(addr_sk),
            rho: Some(rho),
            r: Some(r),
            s: Some(s),
            v: Some(v),
            cm: Some(*cm),
            auth_path: [None; CM_TREE_DEPTH as usize],
        };

        Ok(old_coin)
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

    // pub(crate) fn get_latest_cm_idx(
    //     &self,
    //     // db: &DB,
    // ) -> Result<Option<u128>, WalletError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM)?;

    //     let mut iter = self.db.iterator_cf(&cf, sak_kv_db::IteratorMode::End);

    //     let (cm_idx_bytes, _hash) = match iter.next() {
    //         Some(a) => a,
    //         None => return Ok(None),
    //     };

    //     let cm_idx = sak_kv_db::convert_u8_slice_into_u128(&cm_idx_bytes)?;

    //     Ok(Some(cm_idx))
    // }
}
