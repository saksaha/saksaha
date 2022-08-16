use super::WalletDBSchema;
use crate::db::cfs;
use crate::WalletError;
use sak_crypto::{Scalar, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_proofs::{OldCoin, CM_TREE_DEPTH};
use sak_types::{CoinRecord, CoinStatus, CM};
use type_extension::U8Arr32;

impl WalletDBSchema {
    pub fn get_all_coins(&self) -> Result<Vec<CoinRecord>, WalletError> {
        let iter = self.raw.get_coin_iter()?;

        let mut v = vec![];
        for (_coin_idx, cm) in iter {
            let arr = type_extension::convert_vec_into_u8_32(cm.to_vec())?;
            let cm = ScalarExt::parse_arr(&arr)?;
            let coin = self.get_coin(&cm)?;

            v.push(coin);
        }

        Ok(v)
    }

    pub fn get_coin(&self, cm: &Scalar) -> Result<CoinRecord, WalletError> {
        let addr_pk = match self.raw.get_a_pk(&cm)? {
            Some(p) => p,
            None => return Err(format!("Failed to get a_pk").into()),
        };

        let addr_sk = match self.raw.get_a_sk(&cm)? {
            Some(s) => s,
            None => return Err(format!("Failed to get a_sk").into()),
        };

        let rho = match self.raw.get_rho(&cm)? {
            Some(r) => r,
            None => return Err(format!("Failed to get rho").into()),
        };

        let r = match self.raw.get_r(&cm)? {
            Some(r) => r,
            None => return Err(format!("Failed to get r").into()),
        };

        let s = match self.raw.get_s(&cm)? {
            Some(s) => s,
            None => return Err(format!("Failed to get s").into()),
        };

        let v = match self.raw.get_v(&cm)? {
            Some(v) => v,
            None => return Err(format!("Failed to get v").into()),
        };

        let coin_status = match self.raw.get_coin_status(&cm)? {
            Some(v) => v,
            None => return Err(format!("Failed to get coin_status").into()),
        };

        let coin_idx = match self.raw.get_coin_idx(&cm)? {
            Some(v) => v,
            None => return Err(format!("Failed to get coin_idx").into()),
        };

        let coin_record = CoinRecord {
            addr_pk,
            addr_sk,
            rho,
            r,
            s,
            v,
            cm: *cm,
            coin_status,
            coin_idx: Some(coin_idx),
        };

        Ok(coin_record)
    }

    pub fn put_coin(&self, coin: &CoinRecord) -> Result<(), WalletError> {
        let coin_idx = coin.coin_idx.unwrap_or(
            self.raw.get_latest_coin_idx()?.map(|v| v + 1).unwrap_or(0),
        );

        let mut batch = WriteBatch::default();

        self.raw.batch_put_rho(&mut batch, &coin.cm, &coin.rho)?;

        self.raw.batch_put_r(&mut batch, &coin.cm, &coin.r)?;

        self.raw.batch_put_s(&mut batch, &coin.cm, &coin.s)?;

        self.raw.batch_put_v(&mut batch, &coin.cm, &coin.v)?;

        self.raw
            .batch_put_a_pk(&mut batch, &coin.cm, &coin.addr_pk)?;

        self.raw
            .batch_put_a_sk(&mut batch, &coin.cm, &coin.addr_sk)?;

        self.raw.batch_put_coin_status(
            &mut batch,
            &coin.cm,
            &coin.coin_status,
        )?;

        self.raw
            .batch_put_coin_idx(&mut batch, &coin.cm, &coin_idx)?;

        self.raw.batch_put_cm(&mut batch, &coin_idx, &coin.cm)?;

        self.raw.db.write(batch)?;

        Ok(())
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
