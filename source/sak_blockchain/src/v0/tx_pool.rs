use crate::Hashable;

use super::Transaction;
use std::collections::HashMap;
use tokio::sync::RwLock;

const TX_POOL_CAPACITY: usize = 100;

pub(crate) struct TxPool {
    txs: RwLock<Vec<Transaction>>,
    updated_txs: RwLock<Vec<Transaction>>,
    unique_transactions: RwLock<HashMap<String, Transaction>>,
}

impl TxPool {
    pub(crate) fn new() -> TxPool {
        let (txs, updated_txs, unique_transactions) = {
            let s = HashMap::with_capacity(TX_POOL_CAPACITY);
            let txs = vec![];
            let updated_txs = vec![];

            (RwLock::new(txs), RwLock::new(updated_txs), RwLock::new(s))
        };

        TxPool {
            txs,
            updated_txs,
            unique_transactions,
        }
    }

    pub async fn get_diff(&self) -> Vec<Transaction> {
        let mut tx_set_lock = self.updated_txs.write().await;

        let mut txs = Vec::new();
        for t in tx_set_lock.iter() {
            txs.push(t.clone());
        }

        tx_set_lock.clear();

        txs
    }

    pub async fn get_hash_diff(&self) -> Vec<String> {
        let mut tx_set_lock = self.updated_txs.write().await;

        let mut txs = Vec::new();
        for t in tx_set_lock.iter() {
            let tx_hash = match t.get_hash() {
                Ok(h) => h,
                Err(_) => "".into(),
            };
            txs.push(tx_hash);
        }

        tx_set_lock.clear();

        txs
    }

    pub async fn insert(&self, tx: Transaction) -> bool {
        let tx_hash = match tx.get_hash() {
            Ok(hash) => hash,
            Err(_) => return false,
        };

        let mut tx_map_lock = self.unique_transactions.write().await;
        if let Some(_v) = tx_map_lock.get(&tx_hash) {
            return false;
        } else {
            tx_map_lock.insert(tx_hash, tx.clone());
        };

        let mut txs_lock = self.txs.write().await;
        txs_lock.push(tx.clone());

        let mut updated_txs_lock = self.updated_txs.write().await;
        updated_txs_lock.push(tx);

        true
    }
}
