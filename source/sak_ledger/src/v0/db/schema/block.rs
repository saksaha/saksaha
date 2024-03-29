use crate::BlockEntity;
use crate::{CtrStateUpdate, LedgerCols, LedgerDB, LedgerError, MerkleUpdate};
use sak_kv_db::WriteBatch;
use sak_types::{Block, Tx};

impl LedgerDB {
    pub async fn get_blocks(&self, block_hashes: Vec<&String>) -> Result<Vec<Block>, LedgerError> {
        let mut ret = vec![];
        for block_hash in block_hashes {
            if let Some(b) = self.get_block(block_hash)? {
                ret.push(b);
            }
        }

        Ok(ret)
    }

    pub fn get_block(&self, block_hash: &String) -> Result<Option<Block>, LedgerError> {
        let block_entity: Option<BlockEntity> =
            self.get(LedgerCols::BlockEntity, block_hash.as_bytes())?;

        let block_merkle_rt = self.get(LedgerCols::BlockMerkleRt, block_hash.as_bytes())?;

        match (block_entity, block_merkle_rt) {
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
        let mut batch = WriteBatch::default();

        let block_entity = BlockEntity {
            block_hash: block.get_block_hash().to_string(),
            validator_sig: block.validator_sig.to_owned(),
            witness_sigs: block.witness_sigs.to_owned(),
            tx_hashes: block.tx_hashes.to_owned(),
            created_at: block.created_at.to_owned(),
            block_height: block.block_height,
            merkle_rt: block.merkle_rt,
        };

        self.put(
            &mut batch,
            LedgerCols::BlockEntity,
            block_entity.block_hash.as_bytes(),
            &block_entity,
        )?;

        self.put(
            &mut batch,
            LedgerCols::BlockHash,
            &block_entity.block_height.to_be_bytes(),
            &block_entity.block_hash,
        )?;

        self.put(
            &mut batch,
            LedgerCols::BlockMerkleRt,
            block_entity.block_hash.as_bytes(),
            &block_entity.merkle_rt,
        )?;

        self.put(
            &mut batch,
            LedgerCols::EmptyValue,
            &block_entity.merkle_rt,
            &[0u8; 1],
        )?;

        for tx in txs {
            self.batch_put_tx(&mut batch, tx)?;
        }

        for (ctr_addr, ctr_state) in ctr_state_updates {
            self.put(
                &mut batch,
                LedgerCols::CtrState,
                ctr_addr.as_bytes(),
                ctr_state,
            )?;
        }

        for (loc, node_val) in merkle_updates {
            self.put(&mut batch, LedgerCols::MerkleNode, loc.as_bytes(), node_val)?;
        }

        self.db.write(batch)?;

        Ok(block_entity.block_hash)
    }
}
