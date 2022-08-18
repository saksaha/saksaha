use crate::LedgerError;
use crate::{cfs, LedgerDB};
use sak_crypto::{Bls12, Hasher, Proof, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_kv_db::DB;
use sak_types::{
    Cm, CmIdx, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Sn, Tx,
    TxCtrOp, TxHash, TxHeight, TxType,
};
use type_extension::U8Arr32;

impl LedgerDB {
    pub(crate) fn get_tx_type(
        &self,
        tx_hash: &TxHash,
    ) -> Result<Option<TxType>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_TYPE)?;

        match self.db.get_cf(&cf, tx_hash)? {
            Some(v) => {
                let tx_type = match v.get(0) {
                    Some(t) => TxType::from(*t),
                    None => {
                        return Err(format!("tx type is corrupted").into());
                    }
                };

                return Ok(Some(tx_type));
            }
            None => {
                return Ok(None);
            }
        };
    }

    // pub(crate) fn get_tx_hash_by_height(
    //     &self,
    //     tx_height: &u128,
    // ) -> Result<Option<String>, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_HEIGHT)?;

    //     let key = tx_height.to_be_bytes();

    //     match self.db.get_cf(&cf, key)? {
    //         Some(v) => {
    //             let str = String::from_utf8(v)?;

    //             return Ok(Some(str));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    pub(crate) fn get_tx_created_at(
        &self,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_CREATED_AT)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_data(
        &self,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::DATA)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_author_sig(
        &self,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::AUTHOR_SIG)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_pi(
        &self,
        key: &TxHash,
    ) -> Result<Option<Vec<u8>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::PI)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                return Ok(Some(v));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_ctr_addr(
        &self,
        key: &TxHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CTR_ADDR)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    // pub(crate) fn get_tx_height(
    //     &self,
    //     key: &TxHash,
    // ) -> Result<Option<TxHeight>, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::TX_HEIGHT)?;

    //     match self.db.get_cf(&cf, key)? {
    //         Some(v) => {
    //             let height = type_extension::convert_u8_slice_into_u128(&v)?;

    //             return Ok(Some(height));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    // pub(crate) fn get_cm_idx(
    //     &self,
    //     key: &TxHash,
    // ) -> Result<Option<CmIdx>, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

    //     match self.db.get_cf(&cf, key)? {
    //         Some(v) => {
    //             let height = type_extension::convert_u8_slice_into_u128(&v)?;

    //             return Ok(Some(height));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    pub(crate) fn get_cm_idx_by_cm(
        &self,
        cm: &Cm,
    ) -> Result<Option<CmIdx>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

        match self.db.get_cf(&cf, cm)? {
            Some(v) => {
                let cm_idx = type_extension::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(cm_idx));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_v(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_k(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::K)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_s(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_sn_1(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::SN_1)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_sn_2(
        &self,
        db: &DB,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::SN_2)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_tx_hash_by_sn(
        &self,
        db: &DB,
        key: &Sn,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(db, cfs::TX_HASH_BY_SN)?;

        match db.get_cf(&cf, key)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_1(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_1)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_2(
        &self,
        key: &TxHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_2)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = type_extension::convert_vec_into_u8_32(v)?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn batch_put_tx_type(
        &self,
        batch: &mut WriteBatch,
        tx_hash: &TxHash,
        tx_type: TxType,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_TYPE)?;

        batch.put_cf(&cf, tx_hash, &[tx_type as u8]);

        Ok(())
    }

    pub(crate) fn batch_put_tx_created_at(
        &self,
        batch: &mut WriteBatch,
        block_hash: &TxHash,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_delete_tx_created_at(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_CREATED_AT)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_data(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::DATA)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_data(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::DATA)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_pi(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<u8>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::PI)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_pi(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::PI)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_author_sig(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::AUTHOR_SIG)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_delete_author_sig(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::AUTHOR_SIG)?;

        batch.delete_cf(&cf, key);

        Ok(())
    }

    pub(crate) fn batch_put_ctr_addr(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CTR_ADDR)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    // pub(crate) fn batch_put_tx_height(
    //     &self,
    //     batch: &mut WriteBatch,
    //     tx_hash: &TxHash,
    //     tx_height: &u128,
    // ) -> Result<(), LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::TX_HEIGHT)?;

    //     let v = tx_height.to_be_bytes();

    //     batch.put_cf(&cf, tx_hash, v);

    //     Ok(())
    // }

    // pub(crate) fn batch_put_tx_hash_by_height(
    //     &self,
    //     // db: &DB,
    //     batch: &mut WriteBatch,
    //     tx_height: &u128,
    //     tx_hash: &String,
    // ) -> Result<(), LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_HEIGHT)?;

    //     let v = tx_height.to_be_bytes();

    //     batch.put_cf(&cf, v, tx_hash);

    //     Ok(())
    // }

    pub(crate) fn batch_put_tx_hash_by_sn(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &[u8; 32],
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_SN)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_idx_cm(
        &self,
        batch: &mut WriteBatch,
        cm_idx: &CmIdx,
        cm: &Cm,
    ) -> Result<(), LedgerError> {
        let cm_idx = cm_idx.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM)?;

        batch.put_cf(&cf, cm_idx, cm);

        Ok(())
    }

    pub(crate) fn batch_put_cm_idx_1_cm(
        &self,
        batch: &mut WriteBatch,
        cm_idx: &CmIdx,
        cm: &Cm,
    ) -> Result<(), LedgerError> {
        let cm_idx = cm_idx.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM_1)?;

        batch.put_cf(&cf, cm_idx, cm);

        Ok(())
    }

    pub(crate) fn batch_put_cm_idx_2_cm(
        &self,
        batch: &mut WriteBatch,
        cm_idx: &CmIdx,
        cm: &Cm,
    ) -> Result<(), LedgerError> {
        let cm_idx = cm_idx.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM_2)?;

        batch.put_cf(&cf, cm_idx, cm);

        Ok(())
    }

    pub(crate) fn batch_put_cm_cm_idx(
        &self,
        batch: &mut WriteBatch,
        cm: &Cm,
        cm_idx: &CmIdx,
    ) -> Result<(), LedgerError> {
        let cm_idx = cm_idx.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

        batch.put_cf(&cf, cm, cm_idx);

        Ok(())
    }

    // pub(crate) fn batch_put_cm_cm_idx_2(
    //     &self,
    //     batch: &mut WriteBatch,
    //     cm: &Cm,
    //     cm_idx: CmIdx,
    // ) -> Result<(), LedgerError> {
    //     let cm_idx = cm_idx.to_be_bytes();

    //     let cf = self.make_cf_handle(&self.db, cfs::CM_IDX_2)?;

    //     batch.put_cf(&cf, cm, cm_idx);

    //     Ok(())
    // }

    pub(crate) fn batch_put_v(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::V)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_k(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::K)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_s(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::S)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_1(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &U8Arr32,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::SN_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_sn_2(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::SN_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_1(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_1)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_2(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_2)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    pub(crate) fn batch_put_prf_merkle_rt(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::PRF_MERKLE_RT)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }

    // pub(crate) fn batch_increment_cm_idx(
    //     &self,
    //     batch: &mut WriteBatch,
    //     cm: &Cm,
    // ) -> Result<CmIdx, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

    //     let mut iter = self.db.iterator_cf(&cf, sak_kv_db::IteratorMode::End);

    //     let next_cm_idx = match iter.next() {
    //         Some((cm, cm_idx)) => {
    //             type_extension::convert_u8_slice_into_u128(&cm_idx)? + 1
    //         }
    //         None => 0,
    //     };

    //     let next_cm_idx_bytes =
    //         type_extension::convert_u128_into_u8_slice(next_cm_idx)?;

    //     batch.put_cf(&cf, cm, next_cm_idx_bytes);

    //     Ok(next_cm_idx)
    // }
}
