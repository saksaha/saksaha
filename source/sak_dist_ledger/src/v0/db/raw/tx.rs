use crate::LedgerError;
use crate::{cfs, LedgerDB};
use sak_crypto::{Bls12, ScalarExt};
use sak_kv_db::WriteBatch;
use sak_kv_db::DB;
use sak_proofs::{Hasher, Proof};
use sak_types::{
    Cm, CmIdx, MintTx, MintTxCandidate, PourTx, PourTxCandidate, Sn, Tx,
    TxCtrOp, TxHash, TxHeight, TxType,
};

use serde::Deserialize;
use serde::Serialize;

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

    // pub(crate) fn get_cm_idxes_by_cms(
    //     &self,
    //     cms: Vec<&Cm>,
    // ) -> Result<Option<Vec<CmIdx>>, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM_IDX)?;

    //     match self.db.get_cf(&cf, cm)? {
    //         Some(v) => {
    //             let cm_idx = type_extension::convert_u8_slice_into_u128(&v)?;

    //             return Ok(Some(cm_idx));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

    // pub(crate) fn get_cm(
    //     &self,
    //     key: &TxHash,
    // ) -> Result<Option<[u8; 32]>, LedgerError> {
    //     let cf = self.make_cf_handle(&self.db, cfs::CM)?;

    //     match self.db.get_cf(&cf, key)? {
    //         Some(v) => {
    //             let arr = type_extension::convert_vec_into_u8_32(v)?;

    //             return Ok(Some(arr));
    //         }
    //         None => {
    //             return Ok(None);
    //         }
    //     }
    // }

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

    pub(crate) fn get_sns(
        &self,
        key: &TxHash,
    ) -> Result<Option<Vec<Sn>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::SNS)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = v
                    .chunks(32)
                    .map(|v| type_extension::convert_vec_into_u8_32(v.to_vec()))
                    .collect::<Result<Vec<[u8; 32]>, LedgerError>>()?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_tx_hash_by_sn(
        &self,
        key: &Vec<Sn>,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_SN)?;

        let serialized = key.iter().flatten().copied().collect::<Vec<u8>>();

        match self.db.get_cf(&cf, serialized)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cms(
        &self,
        key: &TxHash,
    ) -> Result<Option<Vec<[u8; 32]>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CMS)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr = v
                    .chunks(32)
                    .map(|v| type_extension::convert_vec_into_u8_32(v.to_vec()))
                    .collect::<Result<Vec<[u8; 32]>, LedgerError>>()?;

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_cm_count(
        &self,
        key: &TxHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CM_COUNT)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let cm_count = type_extension::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(cm_count));
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

    pub(crate) fn batch_put_tx_hash_by_sn(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &Vec<Sn>,
        value: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASH_BY_SN)?;

        let serialized = key.iter().flatten().copied().collect::<Vec<u8>>();

        batch.put_cf(&cf, serialized, value);

        Ok(())
    }

    pub(crate) fn batch_put_cm_idx_cm(
        &self,
        batch: &mut WriteBatch,
        cm_idx: &CmIdx,
        cm: &Cm,
    ) -> Result<(), LedgerError> {
        let cm_idx = cm_idx.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM_IDX_CM)?;

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

    pub(crate) fn batch_put_sns(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<Sn>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::SNS)?;

        let serialized = value.iter().flatten().copied().collect::<Vec<u8>>();

        batch.put_cf(&cf, key, serialized);

        Ok(())
    }

    pub(crate) fn batch_put_cms(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        value: &Vec<[u8; 32]>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::CMS)?;

        let serialized = value.iter().flatten().copied().collect::<Vec<u8>>();

        batch.put_cf(&cf, key, serialized);

        Ok(())
    }

    pub(crate) fn batch_put_cm_count(
        &self,
        batch: &mut WriteBatch,
        key: &TxHash,
        cm_count: &u128,
    ) -> Result<(), LedgerError> {
        let cm_count = cm_count.to_be_bytes();

        let cf = self.make_cf_handle(&self.db, cfs::CM_COUNT)?;

        batch.put_cf(&cf, key, cm_count);

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
}
