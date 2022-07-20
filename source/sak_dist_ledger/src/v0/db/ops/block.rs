use crate::{
    CtrStateUpdate, DistLedger, LedgerDB, LedgerDBSchema, LedgerError,
    MerkleUpdate,
};
use colored::Colorize;
use log::debug;
use sak_kv_db::{WriteBatch, DB};
use sak_types::{Block, Tx};

impl LedgerDB {
    pub(crate) fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        get_block(&self.kv_db.db_instance, &self.schema, block_hash)
    }

    pub(crate) fn get_block_hash_by_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<String>, LedgerError> {
        let db = &self.kv_db.db_instance;

        self.schema.get_block_hash(db, block_height)
    }

    pub(crate) async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        let mut ret = vec![];
        for block_hash in block_hashes {
            match get_block(&self.kv_db.db_instance, &self.schema, block_hash)?
            {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
    }

    pub(crate) async fn put_block(
        &self,
        block: &Block,
        txs: &Vec<Tx>,
        ctr_state_updates: &CtrStateUpdate,
        merkle_updates: &MerkleUpdate,
    ) -> Result<String, LedgerError> {
        println!(
            "data to write, block: {:?}, \ntxs: {:?}, \n
            ctr_state_updates: {:?},\n merkle_updates: {:?}",
            block, txs, ctr_state_updates, merkle_updates
        );

        let db = &self.kv_db.db_instance;

        let mut batch = WriteBatch::default();

        let block_hash = block.get_block_hash();

        self.schema.batch_put_validator_sig(
            db,
            &mut batch,
            block_hash,
            &block.validator_sig,
        )?;

        self.schema.batch_put_witness_sigs(
            db,
            &mut batch,
            block_hash,
            &block.witness_sigs,
        )?;

        self.schema.batch_put_tx_hashes(
            db,
            &mut batch,
            block_hash,
            &block.tx_hashes,
        )?;

        self.schema.batch_put_block_created_at(
            db,
            &mut batch,
            block_hash,
            &block.created_at,
        )?;

        self.schema.batch_put_block_hash(
            db,
            &mut batch,
            &block.block_height,
            block_hash,
        )?;

        self.schema.batch_put_block_cm_count(
            db,
            &mut batch,
            &block.block_height,
            block_hash,
        )?;

        self.schema.batch_put_block_height(
            db,
            &mut batch,
            block_hash,
            &block.block_height,
        )?;

        self.schema.batch_put_merkle_rt(
            db,
            &mut batch,
            block_hash,
            &block.merkle_rt,
        )?;

        for tx in txs {
            self.batch_put_tx(&mut batch, tx)?;
        }

        for ctr_addr in ctr_state_updates.keys() {
            self.schema.batch_put_ctr_state(
                db,
                &mut batch,
                &ctr_addr,
                &ctr_state_updates
                    .get(ctr_addr)
                    .expect("contract state should be exist"),
            )?;
        }

        for (loc, node_val) in merkle_updates {
            self.schema
                .batch_put_merkle_node(db, &mut batch, loc, node_val)?;
        }

        db.write(batch)?;

        return Ok(block_hash.clone());
    }

    pub(crate) async fn get_latest_block_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        let db = &self.kv_db.db_instance;

        let height = self.schema.get_latest_block_height(db)?;

        Ok(height)
    }

    pub(crate) async fn get_total_cm_count(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        let db = &self.kv_db.db_instance;

        let total_cm_count = self.schema.get_total_cm_count(db)?;

        Ok(total_cm_count)
    }
}

fn get_block(
    db: &DB,
    schema: &LedgerDBSchema,
    block_hash: &String,
) -> Result<Option<Block>, LedgerError> {
    let validator_sig = schema.get_validator_sig(db, &block_hash)?;

    let tx_hashes = schema.get_tx_hashes(db, &block_hash)?;

    let witness_sigs = schema.get_witness_sigs(db, &block_hash)?;

    let created_at = schema.get_block_created_at(db, &block_hash)?;

    let block_height = schema.get_block_height(db, &block_hash)?;

    let block_merkle_rt = schema.get_block_merkle_rt(db, &block_hash)?;

    let block_cm_count = schema.get_block_cm_count(db, &block_hash)?;

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
