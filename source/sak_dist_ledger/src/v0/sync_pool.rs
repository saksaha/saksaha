use log::warn;
use sak_types::{Block, Tx, TxCandidate, TxType};
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

const SYNC_POOL_CAPACITY: usize = 100;

pub(crate) struct SyncPool {
    new_blocks: RwLock<HashSet<(u128, String)>>,
    new_tx_hashes: RwLock<HashSet<String>>,
    tx_map: RwLock<HashMap<String, TxCandidate>>,
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

        let height = block.get_height();
        let block_hash = block.get_hash();

        new_blocks_lock.insert((height.to_owned(), block_hash.to_owned()));

        Ok(())
    }

    pub(crate) async fn insert_tx(
        &self,
        tx: TxCandidate,
    ) -> Result<(), String> {
        match tx.get_type() {
            TxType::ContractDeploy => {
                // check functions
                match tx.is_valid_ctr_deploying_tx() {
                    Ok(o) => o,
                    Err(err) => {
                        return Err(format!("Err: {:?}", err));
                    }
                }
            }
            TxType::ContractCall => {}
            TxType::Plain => {}
        };

        let tx_hash = tx.get_hash();

        let mut tx_map_lock = self.tx_map.write().await;

        if tx_map_lock.contains_key(tx_hash) {
            return Err(format!("tx already exist"));
        } else {
            tx_map_lock.insert(tx_hash.clone(), tx.clone());
        };

        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;
        new_tx_hashes_lock.insert(tx_hash.to_string());

        Ok(())
    }

    pub(crate) async fn get_all_txs(&self) -> Result<Vec<TxCandidate>, String> {
        let tx_map_lock = self.tx_map.read().await;

        let tx = tx_map_lock.values().map(|v| v.clone()).collect();

        Ok(tx)
    }

    pub(crate) async fn remove_txs(&self, txs: &Vec<Tx>) -> Result<(), String> {
        let mut tx_map_lock = self.tx_map.write().await;

        for tx in txs {
            tx_map_lock.remove(tx.get_hash());
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
