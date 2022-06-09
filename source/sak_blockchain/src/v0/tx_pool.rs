use log::warn;
use sak_types::{Hashable, Transaction};
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
            tx_map,
        }
    }

    pub async fn get_new_tx_hashes(&self) -> Vec<String> {
        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;

        let v: Vec<_> = new_tx_hashes_lock.drain().collect();
        v
    }

    // Returns hashes of transactions that I do not have
    pub async fn get_tx_pool_diff(
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

    pub async fn insert(&self, tx: Transaction) -> Result<(), String> {
        let tx_hash = match String::from_utf8(tx.contract.clone()) {
            Ok(v) => v,
            Err(err) => {
                return Err(format!("Invalid UTF-8 sequence, err: {}", err))
            }
        };

        let mut tx_map_lock = self.tx_map.write().await;

        if tx_map_lock.contains_key(&tx_hash) {
            return Err(format!("tx already exist"));
        } else {
            tx_map_lock.insert(tx_hash.clone(), tx.clone());
        };

        let mut new_tx_hashes_lock = self.new_tx_hashes.write().await;
        new_tx_hashes_lock.insert(tx_hash);

        Ok(())
    }

    pub async fn get_ack_txs(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<Transaction> {
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

    pub async fn contain_check(
        &self,
        tx_hash_str: String,
    ) -> Result<String, String> {
        let tx_map_lock = self.tx_map.read().await;

        if tx_map_lock.contains_key(&tx_hash_str) {
            return Err(format!("tx already exists : {}", tx_hash_str));
        } else {
            return Ok(format!("tx doesnt exist"));
        };
    }
}
