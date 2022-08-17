use crate::{DistLedgerApis, LedgerError};
use sak_contract_std::Storage;
use sak_proofs::{MerkleTree, CM_TREE_DEPTH};
use sak_types::{Block, BlockHash, BlockHeight, CtrAddr, Tx, TxCandidate};

const GET_BLOCK_HASH_LIST_DEFAULT_SIZE: u128 = 10;

impl DistLedgerApis {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.schema.get_blocks(block_hashes).await
    }

    pub async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        self.ledger_db.schema.get_txs(tx_hashes).await
    }

    pub async fn get_merkle_node(
        &self,
        location: &String,
    ) -> Result<[u8; 32], LedgerError> {
        self.ledger_db.schema.get_merkle_node(location)
    }

    pub async fn get_auth_path(
        &self,
        cm_idx: &u128,
    ) -> Result<Vec<([u8; 32], bool)>, LedgerError> {
        let merkle_tree = MerkleTree::new(CM_TREE_DEPTH);

        let auth_path_idx = merkle_tree.generate_auth_paths(cm_idx.to_owned());

        let mut ret: Vec<([u8; 32], bool)> = Vec::new();

        for (idx, p) in auth_path_idx.iter().enumerate() {
            let key = format!("{}_{}", idx, p.idx);

            let merkle_node = match self.get_merkle_node(&key).await {
                Ok(m) => m,
                Err(err) => {
                    return Err(format!(
                        "Couldn't get the merkle node value, err: {}",
                        err
                    )
                    .into())
                }
            };

            ret.push((merkle_node, p.direction));
        }

        Ok(ret)
    }

    pub async fn get_cm_by_idx(
        &self,
        cm_idx: &u128,
    ) -> Result<Option<String>, LedgerError> {
        self.ledger_db.schema.get_cm_by_idx(cm_idx)
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(BlockHeight, BlockHash)>, LedgerError> {
        let latest_block_height =
            match self.ledger_db.schema.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(&latest_block_height)?
        {
            Some(block_hash) => block_hash.to_string(),
            None => return Ok(None),
        };

        Ok(Some((latest_block_height, latest_block_hash)))
    }

    // rpc
    pub async fn send_tx(
        &self,
        tx_candidate: TxCandidate,
    ) -> Result<(), String> {
        match tx_candidate {
            TxCandidate::Mint(_) => {
                let r = self.sync_pool.insert_tx(tx_candidate).await?;

                Ok(r)
            }
            TxCandidate::Pour(_) => {
                //verify

                let r = self.sync_pool.insert_tx(tx_candidate).await?;

                Ok(r)
            }
        }
    }

    pub async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        self.ledger_db.schema.get_tx(tx_hash).await
    }

    pub fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        self.ledger_db.schema.get_block(block_hash)
        // self.get_block(&self.kv_db.db_instance, &self.schema, block_hash)
    }

    pub async fn get_block_list(
        &self,
        offset: Option<u128>,
        limit: Option<u128>,
    ) -> Result<Vec<Block>, LedgerError> {
        let latest_bh = match self.get_latest_block_height()? {
            Some(bh) => bh,
            None => {
                return Err(format!("Cannot find latest block height").into())
            }
        };

        let upper = match offset {
            Some(bh) => {
                if latest_bh < bh {
                    latest_bh
                } else {
                    bh
                }
            }
            None => latest_bh,
        };

        let limit = match limit {
            Some(l) => l,
            None => GET_BLOCK_HASH_LIST_DEFAULT_SIZE,
        };

        let lower = {
            if upper < limit {
                if upper > GET_BLOCK_HASH_LIST_DEFAULT_SIZE {
                    upper - GET_BLOCK_HASH_LIST_DEFAULT_SIZE + 1
                } else {
                    0
                }
            } else {
                limit
            }
        };

        let mut block_hash_list: Vec<BlockHash> = Vec::new();

        for bh in (lower..=upper).rev().step_by(1) {
            match self.get_block_by_height(&bh).await {
                Ok(maybe_block) => match maybe_block {
                    Some(block) => {
                        block_hash_list.push(block.get_block_hash().to_owned());
                    }
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(format!(
                        "Block hash at height ({}) does not exist",
                        err
                    )
                    .into())
                }
            }
        }

        let block_hash_list_tmp: Vec<&BlockHash> =
            block_hash_list.iter().collect();

        let block_list = match self.get_blocks(block_hash_list_tmp).await {
            Ok(bl) => bl,
            Err(err) => {
                return Err(format!(
                    "some of the block_hashes in ({:?}) is wrong",
                    err
                )
                .into())
            }
        };

        Ok(block_list)
    }

    pub async fn get_entire_block_info_list(
        &self,
    ) -> Result<Vec<(u128, BlockHash)>, LedgerError> {
        let latest_bh = match self.get_latest_block_height()? {
            Some(bh) => bh,
            None => {
                return Err(format!("Cannot find latest block height").into())
            }
        };

        let mut block_hash_list: Vec<(u128, BlockHash)> = Vec::new();

        for bh in (0..=latest_bh).rev().step_by(1) {
            match self.get_block_by_height(&bh).await {
                Ok(maybe_block) => match maybe_block {
                    Some(block) => {
                        block_hash_list
                            .push((bh, block.get_block_hash().to_owned()));
                    }
                    None => {
                        break;
                    }
                },
                Err(err) => {
                    return Err(format!(
                        "Block hash at height ({}) does not exist",
                        err
                    )
                    .into())
                }
            }
        }

        Ok(block_hash_list)
    }

    pub async fn get_block_by_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<Block>, LedgerError> {
        if let Some(block_hash) = self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(block_height)?
        {
            return self.ledger_db.schema.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_latest_block_height()
    }

    pub async fn get_ledger_cm_count(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_ledger_cm_count()
    }

    pub async fn get_latest_tx_height(
        &self,
    ) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.schema.get_latest_tx_height()
    }

    pub async fn get_latest_block_merkle_rt(
        &self,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let latest_block_height =
            match self.ledger_db.schema.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .schema
            .get_block_hash_by_block_height(&latest_block_height)?
        {
            Some(h) => h,
            None => {
                return Err(format!(
                    "Block hash at height ({}) does not exist",
                    latest_block_height
                )
                .into())
            }
        };

        let latest_merkle_rt = self
            .ledger_db
            .schema
            .get_block_merkle_rt(&latest_block_hash)?;

        Ok(latest_merkle_rt)
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &CtrAddr,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.schema.get_ctr_state(contract_addr)
    }
}
