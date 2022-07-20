use crate::{cfs, CtrStateUpdate, LedgerDBSchema, MerkleUpdate};
use crate::{LedgerError, MerkleNodeLoc};
use sak_kv_db::DB;
use sak_kv_db::{
    BoundColumnFamily, ColumnFamilyDescriptor, IteratorMode, Options,
    WriteBatch,
};
use sak_types::{Block, BlockHash, CtrAddr, Tx, TxHash, TxType};
use std::convert::TryInto;
use std::sync::Arc;

// getter
impl LedgerDBSchema {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        let mut ret = vec![];
        for block_hash in block_hashes {
            match self.get_block(
                // &self.kv_db.db_instance,
                // &self.schema,
                block_hash,
            )? {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
    }

    pub fn get_block(
        // schema: &LedgerDBSchema,
        &self,
        // db: &DB,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        let validator_sig = self.get_validator_sig(&block_hash)?;

        let tx_hashes = self.get_tx_hashes(&block_hash)?;

        let witness_sigs = self.get_witness_sigs(&block_hash)?;

        let created_at = self.get_block_created_at(&block_hash)?;

        let block_height = self.get_block_height(&block_hash)?;

        let block_merkle_rt = self.get_block_merkle_rt(&block_hash)?;

        let block_cm_count = self.get_block_cm_count(&block_hash)?;

        match (
            validator_sig,
            tx_hashes,
            witness_sigs,
            created_at,
            block_height,
            block_merkle_rt,
            block_cm_count,
        ) {
            (
                Some(vs),
                Some(th),
                Some(ws),
                Some(ca),
                Some(bh),
                Some(mr),
                Some(bcc),
            ) => {
                let b = Block::new(vs, th, ws, ca, bh, mr, bcc);
                return Ok(Some(b));
            }
            (None, None, None, None, None, None, None) => {
                return Ok(None);
            }
            _ => {
                return Err(format!(
                    "Block is corrupted. Some data is missing, block_hash: {}",
                    block_hash,
                )
                .into());
            }
        }
    }

    pub(crate) fn get_validator_sig(
        &self,
        // db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::VALIDATOR_SIG)?;

        match self.db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        };
    }

    pub(crate) fn get_tx_hashes(
        &self,
        // db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASHES)?;

        match self.db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                return Ok(Some(th));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_witness_sigs(
        &self,
        // db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<Vec<String>>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::WITNESS_SIGS)?;

        match self.db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let th: Vec<String> = serde_json::from_slice(&v).unwrap();
                return Ok(Some(th));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_created_at(
        &self,
        // db: &DB,
        key: &BlockHash,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_CREATED_AT)?;

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

    pub(crate) fn get_block_height(
        &self,
        // db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_HEIGHT)?;

        match self.db.get_cf(&cf, block_hash)? {
            Some(h) => {
                let height = sak_kv_db::convert_u8_slice_into_u128(&h)?;

                return Ok(Some(height));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_hash_by_block_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_HASH)?;

        let v = block_height.to_be_bytes();

        match self.db.get_cf(&cf, v)? {
            Some(v) => {
                let str = String::from_utf8(v)?;

                return Ok(Some(str));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_cm_count(
        &self,
        // db: &DB,
        key: &BlockHash,
    ) -> Result<Option<u128>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_CM_COUNT)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let val = sak_kv_db::convert_u8_slice_into_u128(&v)?;

                return Ok(Some(val));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_block_merkle_rt(
        &self,
        // db: &DB,
        block_hash: &BlockHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_MERKLE_RT)?;

        match self.db.get_cf(&cf, block_hash)? {
            Some(v) => {
                let arr: [u8; 32] = match v.try_into() {
                    Ok(a) => a,
                    Err(_) => {
                        return Err(
                            format!("Cannot convert cm into an array",).into()
                        )
                    }
                };

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }

    pub(crate) fn get_prf_merkle_rt(
        &self,
        // db: &DB,
        key: &BlockHash,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::PRF_MERKLE_RT)?;

        match self.db.get_cf(&cf, key)? {
            Some(v) => {
                let arr: [u8; 32] = match v.try_into() {
                    Ok(a) => a,
                    Err(err) => {
                        return Err(
                            format!("Cannot convert cm into an array",).into()
                        )
                    }
                };

                return Ok(Some(arr));
            }
            None => {
                return Ok(None);
            }
        }
    }
}

// writer
impl LedgerDBSchema {
    pub(crate) async fn put_block(
        &self,
        block: &Block,
        txs: &Vec<Tx>,
        ctr_state_updates: &CtrStateUpdate,
        merkle_updates: &MerkleUpdate,
        updated_ledger_cm_count: u128,
    ) -> Result<String, LedgerError> {
        // println!(
        //     "data to write, block: {:?}, \ntxs: {:?},\n\
        //     ctr_state_updates: {:?},\n merkle_updates: {:?}, \n\
        //     updated_ledger_cm_count: {}",
        //     block,
        //     txs,
        //     ctr_state_updates,
        //     merkle_updates,
        //     updated_ledger_cm_count,
        // );

        // let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        let block_hash = block.get_block_hash();

        self.batch_put_validator_sig(
            // db,
            &mut batch,
            block_hash,
            &block.validator_sig,
        )?;

        self.batch_put_witness_sigs(
            // db,
            &mut batch,
            block_hash,
            &block.witness_sigs,
        )?;

        self.batch_put_tx_hashes(
            // db,
            &mut batch,
            block_hash,
            &block.tx_hashes,
        )?;

        self.batch_put_block_created_at(
            // db,
            &mut batch,
            block_hash,
            &block.created_at,
        )?;

        self.batch_put_block_hash(
            // db,
            &mut batch,
            &block.block_height,
            block_hash,
        )?;

        self.batch_put_block_cm_count(
            // db,
            &mut batch,
            block_hash,
            block.block_cm_count,
        )?;

        self.batch_put_ledger_cm_count(
            // db,
            &mut batch,
            updated_ledger_cm_count,
        )?;

        self.batch_put_block_height(
            // db,
            &mut batch,
            block_hash,
            &block.block_height,
        )?;

        self.batch_put_block_merkle_rt(
            // db,
            &mut batch,
            block_hash,
            &block.merkle_rt,
        )?;

        for tx in txs {
            self.batch_put_tx(&mut batch, tx)?;
        }

        for (ctr_addr, ctr_state) in ctr_state_updates {
            self.batch_put_ctr_state(&mut batch, ctr_addr, ctr_state)?;
        }

        for (loc, node_val) in merkle_updates {
            self.batch_put_merkle_node(
                // db,
                &mut batch, loc, node_val,
            )?;
        }

        self.db.write(batch)?;

        return Ok(block_hash.clone());
    }
    pub(crate) fn batch_put_validator_sig(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        validator_sig: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::VALIDATOR_SIG)?;

        batch.put_cf(&cf, block_hash, validator_sig);

        Ok(())
    }

    pub(crate) fn batch_put_witness_sigs(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        witness_sigs: &Vec<String>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::WITNESS_SIGS)?;

        let witness_sigs = serde_json::to_string(witness_sigs)?;

        batch.put_cf(&cf, block_hash, witness_sigs);

        Ok(())
    }

    pub(crate) fn batch_put_tx_hashes(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        tx_hashes: &Vec<String>,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::TX_HASHES)?;

        let transactions = serde_json::to_string(tx_hashes)?;

        batch.put_cf(&cf, block_hash, transactions);

        Ok(())
    }

    pub(crate) fn batch_put_block_created_at(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        created_at: &String,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_CREATED_AT)?;

        batch.put_cf(&cf, block_hash, created_at);

        Ok(())
    }

    pub(crate) fn batch_put_block_cm_count(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        cm_count: u128,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_CM_COUNT)?;

        let v = cm_count.to_be_bytes();

        batch.put_cf(&cf, block_hash, &v);

        Ok(())
    }

    pub(crate) fn batch_put_block_hash(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_height: &u128,
        block_hash: &BlockHash,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_HASH)?;

        let v = block_height.to_be_bytes();

        batch.put_cf(&cf, &v, block_hash);

        Ok(())
    }

    pub(crate) fn batch_put_block_height(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        block_hash: &BlockHash,
        block_height: &u128,
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_HEIGHT)?;

        let v = block_height.to_be_bytes();

        batch.put_cf(&cf, block_hash, v);

        Ok(())
    }

    pub(crate) fn batch_put_block_merkle_rt(
        &self,
        // db: &DB,
        batch: &mut WriteBatch,
        key: &BlockHash,
        value: &[u8; 32],
    ) -> Result<(), LedgerError> {
        let cf = self.make_cf_handle(&self.db, cfs::BLOCK_MERKLE_RT)?;

        batch.put_cf(&cf, key, value);

        Ok(())
    }
}
