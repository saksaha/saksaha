use crate::{DistLedgerApis, LedgerError};
use sak_contract_std::Storage;
use sak_proofs::{MerkleTree, CM_TREE_DEPTH};
use sak_types::{
    Block, BlockHash, BlockHeight, Cm, CmIdx, CtrAddr, Tx, TxCandidate, TxHash,
};

const GET_BLOCK_HASH_LIST_DEFAULT_SIZE: u128 = 10;

impl DistLedgerApis {
    pub async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerError> {
        self.ledger_db.get_blocks(block_hashes).await
    }

    pub async fn get_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<Tx>, LedgerError> {
        self.ledger_db.get_txs(tx_hashes).await
    }

    pub async fn get_merkle_node(
        &self,
        location: &String,
    ) -> Result<[u8; 32], LedgerError> {
        self.ledger_db.get_merkle_node(location)
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

    // pub async fn get_cm_by_idx(
    //     &self,
    //     cm_idx: &CmIdx,
    // ) -> Result<Option<Cm>, LedgerError> {
    //     self.ledger_db.get_cm_by_cm_idx(cm_idx)
    // }

    pub async fn get_cm_idx_by_cm(
        &self,
        cm: &Cm,
    ) -> Result<Option<CmIdx>, LedgerError> {
        self.ledger_db.get_cm_idx_by_cm(cm)
    }

    pub async fn get_latest_block_hash(
        &self,
    ) -> Result<Option<(BlockHeight, BlockHash)>, LedgerError> {
        let latest_block_height =
            match self.ledger_db.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
            .get_block_hash_by_block_height(&latest_block_height)?
        {
            Some(block_hash) => block_hash.to_string(),
            None => return Ok(None),
        };

        Ok(Some((latest_block_height, latest_block_hash)))
    }

    pub async fn send_tx(
        &self,
        tx_candidate: TxCandidate,
    ) -> Result<TxHash, LedgerError> {
        let tx_hash = match tx_candidate.clone() {
            TxCandidate::Mint(_) => {
                self.sync_pool.insert_tx(tx_candidate).await?
            }
            TxCandidate::Pour(tc) => {
                let is_valid_sn = self.verify_sn(&tc.sn_1);
                let is_verified_tx = self.verify_proof(&tc)?;

                if is_valid_sn & is_verified_tx {
                    self.sync_pool.insert_tx(tx_candidate).await?
                } else {
                    return Err(format!("tc is not valid").into());
                }
            }
        };

        Ok(tx_hash)
    }

    pub async fn get_tx(
        &self,
        tx_hash: &String,
    ) -> Result<Option<Tx>, LedgerError> {
        self.ledger_db.get_tx(tx_hash).await
    }

    pub fn get_block(
        &self,
        block_hash: &String,
    ) -> Result<Option<Block>, LedgerError> {
        self.ledger_db.get_block(block_hash)
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

    pub async fn get_all_blocks(
        &self,
    ) -> Result<Vec<(BlockHeight, BlockHash)>, LedgerError> {
        let latest_bh = match self.get_latest_block_height()? {
            Some(bh) => bh,
            None => {
                return Err(format!("Cannot find latest block height").into())
            }
        };

        let mut block_hash_list: Vec<(BlockHeight, BlockHash)> = Vec::new();

        for block_height in (0..=latest_bh).rev().step_by(1) {
            match self.get_block_by_height(&block_height).await {
                Ok(maybe_block) => match maybe_block {
                    Some(block) => {
                        block_hash_list.push((
                            block_height,
                            block.get_block_hash().to_string(),
                        ));
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
            .get_block_hash_by_block_height(block_height)?
        {
            return self.ledger_db.get_block(&block_hash);
        } else {
            return Ok(None);
        }
    }

    pub fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerError> {
        self.ledger_db.get_latest_block_height()
    }

    // pub async fn get_ledger_cm_count(
    //     &self,
    // ) -> Result<Option<u128>, LedgerError> {
    //     self.ledger_db.get_ledger_cm_count()
    // }

    // pub async fn get_latest_tx_height(
    //     &self,
    // ) -> Result<Option<u128>, LedgerError> {
    //     self.ledger_db.get_latest_tx_height()
    // }

    pub async fn get_latest_block_merkle_rt(
        &self,
    ) -> Result<Option<[u8; 32]>, LedgerError> {
        let latest_block_height =
            match self.ledger_db.get_latest_block_height()? {
                Some(h) => h,
                None => return Ok(None),
            };

        let latest_block_hash = match self
            .ledger_db
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

        let latest_merkle_rt =
            self.ledger_db.get_block_merkle_rt(&latest_block_hash)?;

        Ok(latest_merkle_rt)
    }

    pub async fn get_ctr_state(
        &self,
        contract_addr: &CtrAddr,
    ) -> Result<Option<Storage>, LedgerError> {
        self.ledger_db.get_ctr_state(contract_addr)
    }
}
