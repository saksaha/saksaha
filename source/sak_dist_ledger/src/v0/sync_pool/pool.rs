use crate::DistLedgerEvent;
use log::{debug, warn};
use sak_types::{Block, BlockHash, BlockHeight, TxCandidate, TxCtrOp, TxHash};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    time::Duration,
};
use tokio::sync::{broadcast::Sender, RwLock};

const SYNC_POOL_CAPACITY: usize = 100;
const TX_SYNC_INTERVAL: u64 = 2000;
const BLOCK_SYNC_INTERVAL: u64 = 2000;

pub(crate) struct SyncPool {
    new_blocks: Arc<RwLock<HashSet<(BlockHeight, BlockHash)>>>,
    tx_hash_set: Arc<RwLock<HashSet<TxHash>>>,
    tx_map: RwLock<HashMap<TxHash, TxCandidate>>,
    ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    tx_sync_interval: Duration,
    block_sync_interval: Duration,
}

impl SyncPool {
    pub(crate) fn new(
        ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
        tx_sync_interval: Option<u64>,
        block_sync_interval: Option<u64>,
    ) -> SyncPool {
        let tx_hash_set = {
            let s = HashSet::new();

            Arc::new(RwLock::new(s))
        };

        let new_blocks = {
            let s = HashSet::new();

            Arc::new(RwLock::new(s))
        };

        let tx_map = {
            let m = HashMap::with_capacity(SYNC_POOL_CAPACITY);
            RwLock::new(m)
        };

        let tx_sync_interval = match tx_sync_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(TX_SYNC_INTERVAL),
        };

        let block_sync_interval = match block_sync_interval {
            Some(i) => Duration::from_millis(i.into()),
            None => Duration::from_millis(BLOCK_SYNC_INTERVAL),
        };

        SyncPool {
            new_blocks,
            tx_hash_set,
            tx_map,
            ledger_event_tx,
            tx_sync_interval,
            block_sync_interval,
        }
    }

    pub(crate) async fn drain_new_blocks(&self) -> Vec<(u128, String)> {
        let mut new_blocks_lock = self.new_blocks.write().await;

        let v: Vec<_> = new_blocks_lock.drain().collect();
        v
    }

    pub(crate) async fn drain_new_tx_hashes(&self) -> Vec<String> {
        let mut new_tx_hashes_lock = self.tx_hash_set.write().await;

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
        let height = block.block_height;
        let block_hash = block.get_block_hash();

        {
            let mut new_blocks_lock = self.new_blocks.write().await;
            new_blocks_lock.insert((height.to_owned(), block_hash.to_owned()));
        }

        let new_blokcs_len_check = self.new_blocks.read().await.len();

        if new_blokcs_len_check == 1 {
            let new_blocks_set = self.new_blocks.clone();

            let ledger_event_tx = self.ledger_event_tx.clone();

            let block_interval = self.block_sync_interval;

            tokio::spawn(async move {
                tokio::time::sleep(block_interval).await;

                let tx_hashes = new_blocks_set.write().await.drain().collect();

                let ev = DistLedgerEvent::NewBlocks(tx_hashes);

                match ledger_event_tx.send(ev.clone()) {
                    Ok(_) => {
                        debug!("Ledger event queued, ev: {}", ev.to_string());
                    }
                    Err(err) => {
                        warn!(
                            "No active tx sync routine receiver handle to \
                                        sync tx event, \
                                    err: {}",
                            err
                        );
                    }
                };
            });
        }
        Ok(())
    }

    pub(crate) async fn insert_tx(
        &self,
        tc: TxCandidate,
    ) -> Result<TxHash, String> {
        // for test,

        {
            // Check if tx is valid ctr deploying type
            // let (tx_ctr_op, tx_coin_op) = tc.get_tx_op();

            let tx_ctr_op = tc.get_ctr_op();

            match tx_ctr_op {
                TxCtrOp::ContractDeploy => {
                    // check functions
                    let maybe_wasm = tc.get_data();
                    if !sak_vm::is_valid_wasm(maybe_wasm) {
                        return Err("Not valid wasm data".to_string());
                    }
                }
                TxCtrOp::ContractCall => {
                    //
                }
                TxCtrOp::None => {}
            };
        }

        let tx_hash = tc.get_tx_hash().to_string();

        {
            let mut tx_map_lock = self.tx_map.write().await;

            if tx_map_lock.contains_key(&tx_hash) {
                return Err("tx already exist".to_string());
            } else {
                tx_map_lock.insert(tx_hash.clone(), tc.clone());
            };
        }

        {
            let mut tx_hashes_set_lock = self.tx_hash_set.write().await;
            tx_hashes_set_lock.insert(tx_hash.to_string());
        }

        // ----------------------------------------------------------

        if self.tx_hash_set.read().await.len() == 1 {
            let new_tx_hashes = self.tx_hash_set.clone();

            let ledger_event_tx = self.ledger_event_tx.clone();

            let tx_interval = self.tx_sync_interval;

            tokio::spawn(async move {
                tokio::time::sleep(tx_interval).await;

                let tx_hashes: Vec<String> =
                    new_tx_hashes.write().await.drain().collect();

                let ev = DistLedgerEvent::TxPoolStat(tx_hashes);

                match ledger_event_tx.send(ev.clone()) {
                    Ok(_) => {
                        debug!("Ledger event queued, ev: {}", ev.to_string());
                    }
                    Err(err) => {
                        warn!(
                            "No active tx sync routine receiver handle to \
                                        sync tx event, \
                                    err: {}",
                            err
                        );
                    }
                };
            });
        }

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
                    warn!("Requested tx does not exist\n");
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
