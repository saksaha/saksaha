use log::warn;
use sak_types::{Hashable, Transaction};
use std::{
    collections::{HashMap, HashSet},
    iter::FromIterator,
};
use tokio::sync::{
    mpsc::{self, Receiver, Sender},
    Mutex, RwLock,
};

const TX_POOL_CAPACITY: usize = 100;

pub(crate) struct TxPool {
    new_tx_hashes: RwLock<HashSet<String>>,
    tx_map: RwLock<HashMap<String, Transaction>>,
    tx_pool_event_rx: RwLock<Receiver<usize>>,
    tx_pool_event_tx: Sender<usize>,
    // has_update_ev_queued: Mutex<>,
}

impl TxPool {
    pub(crate) fn new() -> TxPool {
        let (tx_pool_event_tx, tx_pool_event_rx) = {
            let (tx, rx) = mpsc::channel::<usize>(1);

            (tx, RwLock::new(rx))
        };

        let new_tx_hashes = {
            let s = HashSet::new();

            RwLock::new(s)
        };

        let tx_map = {
            let m = HashMap::with_capacity(TX_POOL_CAPACITY);
            RwLock::new(m)
        };

        let has_update_ev_queued = { Mutex::new(false) };

        TxPool {
            new_tx_hashes,
            tx_map,
            tx_pool_event_tx,
            tx_pool_event_rx,
            has_update_ev_queued,
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

    pub async fn next_update(&self) {
        let mut _tx_pool_event_rx_lock =
            self.tx_pool_event_rx.write().await.recv().await;
        // tx_pool_event_rx_lock.recv
    }

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

        match self.has_update_ev_queued.try_lock() {
            Ok(mut i) => {
                *i = true;
                tokio::spawn(async {
                    // sleep
                })
            }
            Err(_) => (),
        };

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
}

pub struct TxPoolUpdateEvent {}
