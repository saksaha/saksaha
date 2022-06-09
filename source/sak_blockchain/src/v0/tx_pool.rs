use crate::Hashable;

use super::Transaction;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

const TX_POOL_CAPACITY: usize = 100;

pub(crate) struct TxPool {
    new_tx_hashes: RwLock<HashSet<String>>,
    tx_map: RwLock<HashMap<String, Transaction>>,
}

impl TxPool {
    pub(crate) fn new() -> TxPool {
        let new_tx_hashes = {
            let s = HashSet::new();

            RwLock::new(s)
        };

        let tx_map = {
            let m = HashMap::with_capacity(TX_POOL_CAPACITY);
            RwLock::new(m)
        };

        TxPool {
            new_tx_hashes,
            // txs,
            // updated_txs,
            tx_map,
        }
    }

    pub async fn get_new_tx_hashes(&self) -> Vec<String> {
        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;

        let v: Vec<_> = new_tx_hashes_lock.drain().collect();
        v
    }

    // pub async fn get_hash_diff(&self) -> Vec<String> {
    //     let mut tx_set_lock = self.updated_txs.write().await;

    //     let mut txs = Vec::new();
    //     for t in tx_set_lock.iter() {
    //         let tx_hash = match t.get_hash() {
    //             Ok(h) => h,
    //             Err(_) => "".into(),
    //         };
    //         txs.push(tx_hash);
    //     }

    //     tx_set_lock.clear();

    //     txs
    // }

    pub async fn insert(&self, tx: Transaction) -> Result<(), String> {
        let tx_hash = tx.get_hash()?;

        let mut tx_map_lock = self.tx_map.write().await;

        if tx_map_lock.contains_key(&tx_hash) {
            return Err(format!("tx already exist"));
        } else {
            tx_map_lock.insert(tx_hash.clone(), tx.clone());
        };

        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;
        new_tx_hashes_lock.insert(tx_hash);

        // let mut txs_lock = self.txs.write().await;
        // txs_lock.push(tx.clone());

        // let mut updated_txs_lock = self.updated_txs.write().await;
        // updated_txs_lock.push(tx);

        Ok(())
    }
}
