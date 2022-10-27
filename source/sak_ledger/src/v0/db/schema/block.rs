use crate::{cfs, CFSenum, CtrStateUpdate, LedgerDB, MerkleUpdate};
use crate::{BlockEntity, MachineError};
use sak_kv_db::WriteBatch;
use sak_types::{Block, BlockHash, BlockHeight, Tx};

impl LedgerDB {
    pub async fn get_blocks(&self, block_hashes: Vec<&String>) -> Result<Vec<Block>, LedgerError> {
        let mut ret = vec![];
        for block_hash in block_hashes {
            match self.get_block(block_hash)? {
                Some(b) => ret.push(b),
                None => (),
            }
        }

        Ok(ret)
    }

    pub fn get_block(&self, block_hash: &String) -> Result<Option<Block>, MachineError> {
        // let validator_sig = self.get_validator_sig(&block_hash)?;

        // let tx_hashes = self.get_tx_hashes(&block_hash)?;

        // let witness_sigs = self.get_witness_sigs(&block_hash)?;

        // let created_at = self.get_block_created_at(&block_hash)?;

        // let block_height = self.get_block_height(&block_hash)?;

        let block_entity: Option<BlockEntity> =
            self.get_ser(CFSenum::BlockEntity, block_hash.as_bytes())?;

        let block_merkle_rt = self.get_block_merkle_rt(block_hash)?;

        match (
            // block_entity.validator_sig,
            // block_entity.tx_hashes,
            // block_entity.witness_sigs,
            // block_entity.created_at,
            // block_entity.block_height,
            block_entity,
            block_merkle_rt,
        ) {
            (Some(b), Some(mr)) => {
                let b = Block::new(
                    b.validator_sig,
                    b.tx_hashes,
                    b.witness_sigs,
                    b.created_at,
                    b.block_height,
                    mr,
                );

                Ok(Some(b))
            }
            (None, None) => Ok(None),
            _ => Err(format!(
                "Block is corrupted. Some data is missing, block_hash: {}",
                block_hash,
            )
            .into()),
        }
    }

    pub async fn put_block(
        &self,
        block: &Block,
        txs: &Vec<Tx>,
        ctr_state_updates: &CtrStateUpdate,
        merkle_updates: &MerkleUpdate,
    ) -> Result<String, LedgerError> {
        println!("block to write, block: {:?}", block,);

        let mut batch = WriteBatch::default();

        // let block_hash = block.get_block_hash();

        let block_entity = BlockEntity {
            block_hash: block.get_block_hash().to_string(),
            validator_sig: block.validator_sig.to_owned(),
            witness_sigs: block.witness_sigs.to_owned(),
            tx_hashes: block.tx_hashes.to_owned(),
            created_at: block.created_at.to_owned(),
            block_height: block.block_height,
            merkle_rt: block.merkle_rt,
        };

        self.put_ser(
            &mut batch,
            CFSenum::BlockEntity,
            block_entity.block_hash.as_bytes(),
            &block_entity,
        )?;

        // self.batch_put_validator_sig(&mut batch, block_hash, &block.validator_sig)?;

        // self.batch_put_witness_sigs(&mut batch, block_hash, &block.witness_sigs)?;

        // self.batch_put_tx_hashes(&mut batch, block_hash, &block.tx_hashes)?;

        // self.batch_put_block_created_at(&mut batch, block_hash, &block.created_at)?;

        self.batch_put_block_hash(&mut batch, &block.block_height, &block_entity.block_hash)?;

        // self.batch_put_block_height(&mut batch, block_hash, &block.block_height)?;

        self.batch_put_block_merkle_rt(&mut batch, &block_entity.block_hash, &block.merkle_rt)?;

        self.batch_put_block_merkle_rt_key(&mut batch, &block.merkle_rt)?;

        for tx in txs {
            self.batch_put_tx(&mut batch, tx)?;
        }

        for (ctr_addr, ctr_state) in ctr_state_updates {
            self.batch_put_ctr_state(&mut batch, ctr_addr, ctr_state)?;
        }

        for (loc, node_val) in merkle_updates {
            self.batch_put_merkle_node(&mut batch, loc, node_val)?;
        }

        self.db.write(batch)?;

        Ok(block_entity.block_hash)
    }
}
