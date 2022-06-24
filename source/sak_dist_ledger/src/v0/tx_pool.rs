use log::{error, warn};
use sak_types::Tx;
use std::collections::{HashMap, HashSet};
use tokio::sync::RwLock;

const TX_POOL_CAPACITY: usize = 100;

pub(crate) struct TxPool {
    new_tx_hashes: RwLock<HashSet<String>>,
    tx_map: RwLock<HashMap<String, Tx>>,
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
            tx_map,
        }
    }

    pub(crate) async fn get_new_tx_hashes(&self) -> Vec<String> {
        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;

        let v: Vec<_> = new_tx_hashes_lock.drain().collect();
        v
    }

    pub(crate) async fn get_tx_pool(&self) -> (Vec<String>, Vec<Tx>) {
        let tx_map_lock = self.tx_map.read().await;
        let mut hashes = vec![];
        let mut txs = vec![];

        for (k, v) in tx_map_lock.iter() {
            hashes.push(k.clone());
            txs.push(v.clone());
        }

        (hashes, txs)
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

    pub(crate) async fn insert(&self, tx: Tx) -> Result<(), String> {
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

    pub(crate) async fn remove_txs(
        &self,
        tx_hashes: &Vec<String>,
    ) -> Result<Vec<String>, String> {
        let mut tx_map_lock = self.tx_map.write().await;
        let mut diff_hashes = vec![];

        for tx_hash in tx_hashes.iter() {
            if tx_map_lock.contains_key(tx_hash) {
                let _tx = match tx_map_lock.remove(tx_hash) {
                    Some(v) => v,
                    None => {
                        return Err(format!(
                            "Can't remove tx having the tx_hash :{}",
                            tx_hash
                        ))
                    }
                };
            } else {
                diff_hashes.push(tx_hash.to_string());
            };
        }

        Ok(diff_hashes)
    }

    pub(crate) async fn get_txs(&self, tx_hashes: Vec<String>) -> Vec<Tx> {
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

    pub(crate) async fn contains(&self, tx_hash: &String) -> bool {
        let tx_map_lock = self.tx_map.read().await;

        tx_map_lock.contains_key(tx_hash)
    }
}
