use log::warn;
use sak_types::{
    Block, BlockHash, BlockHeight, Tx, TxCandidate, TxCtrOp, TxHash,
};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

const SYNC_POOL_CAPACITY: usize = 100;

pub(crate) struct SyncPool {
    new_blocks: RwLock<HashSet<(BlockHeight, BlockHash)>>,
    new_tx_hashes: RwLock<HashSet<TxHash>>,
    tx_map: RwLock<HashMap<TxHash, TxCandidate>>,
}

impl SyncPool {
    pub(crate) fn new() -> SyncPool {
        let new_tx_hashes = {
            let s = HashSet::new();

            RwLock::new(s)
        };

        let new_blocks = {
            let s = HashSet::new();

            RwLock::new(s)
        };

        let tx_map = {
            let m = HashMap::with_capacity(SYNC_POOL_CAPACITY);
            RwLock::new(m)
        };

        SyncPool {
            new_blocks,
            new_tx_hashes,
            tx_map,
        }
    }

    pub(crate) async fn drain_new_blocks(&self) -> Vec<(u128, String)> {
        let mut new_blocks_lock = self.new_blocks.write().await;

        let v: Vec<_> = new_blocks_lock.drain().collect();
        v
    }

    pub(crate) async fn drain_new_tx_hashes(&self) -> Vec<String> {
        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;

        let v: Vec<_> = new_tx_hashes_lock.drain().collect();
        v
    }

    // Returns hashes of transactions that I do not have
    pub(crate) async fn get_tx_pool_diff(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        let tx_map_lock = self.tx_map.write().await;

        let mut ret = vec![];

        for h in tx_hashes {
            if !tx_map_lock.contains_key(&h) {
                ret.push(h.clone());
            }
        }

        return ret;
    }

    pub(crate) async fn insert_block(
        &self,
        block: &Block,
    ) -> Result<(), String> {
        let mut new_blocks_lock = self.new_blocks.write().await;

        let height = block.block_height;
        let block_hash = block.get_block_hash();

        new_blocks_lock.insert((height.to_owned(), block_hash.to_owned()));

        Ok(())
    }

    pub(crate) async fn insert_tx(
        &self,
        tc: TxCandidate,
    ) -> Result<TxHash, String> {
        {
            // Check if tx is valid ctr deploying type
            // let (tx_ctr_op, tx_coin_op) = tc.get_tx_op();
            let tx_ctr_op = tc.get_ctr_op();

            match tx_ctr_op {
                TxCtrOp::ContractDeploy => {
                    // check functions
                    let maybe_wasm = tc.get_data();
                    if !sak_vm::is_valid_wasm(maybe_wasm) {
                        return Err(format!("Not valid wasm data"));
                    }
                }
                TxCtrOp::ContractCall => {
                    //
                }
                TxCtrOp::None => {}
            };
        }

        let tx_hash = tc.get_tx_hash().to_string();

        let mut tx_map_lock = self.tx_map.write().await;

        if tx_map_lock.contains_key(&tx_hash) {
            return Err(format!("tx already exist"));
        } else {
            tx_map_lock.insert(tx_hash.clone(), tc);
        };

        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;

        new_tx_hashes_lock.insert(tx_hash.to_string());

        Ok(tx_hash)
    }

    pub(crate) async fn get_all_txs(&self) -> Result<Vec<TxCandidate>, String> {
        let tx_map_lock = self.tx_map.read().await;

        let txs = tx_map_lock.values().map(|v| v.clone()).collect();

        Ok(txs)
    }

    pub(crate) async fn remove_tcs(
        &self,
        txs: &Vec<TxCandidate>,
    ) -> Result<(), String> {
        let mut tx_map_lock = self.tx_map.write().await;

        for tx in txs {
            tx_map_lock.remove(tx.get_tx_hash());
        }

        Ok(())
    }

    pub(crate) async fn get_txs(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<TxCandidate> {
        let tx_map_lock = self.tx_map.read().await;
        let mut tx_pool = vec![];

        for tx_hash in tx_hashes.iter() {
            let tx = match tx_map_lock.get(tx_hash) {
                Some(tx) => tx.clone(),
                None => {
                    warn!("Requested tx does not exist");
                    continue;
                }
            };

            tx_pool.push(tx);
        }

        tx_pool
    }

    pub(crate) async fn contains_tx(&self, tx_hash: &String) -> bool {
        let tx_map_lock = self.tx_map.read().await;

        tx_map_lock.contains_key(tx_hash)
    }
}
