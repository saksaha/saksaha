use crate::{DistLedgerApis, LedgerError};
use log::warn;
use sak_types::{BlockCandidate, TxCandidate};

impl DistLedgerApis {
    // peer_node
    pub async fn insert_into_pool(&self, tx_candidates: Vec<TxCandidate>) {
        for tx in tx_candidates.into_iter() {
            if let Err(err) = self.sync_pool.insert_tx(tx).await {
                warn!("Tx pool insertion aborted, reason: {}", err);
            };
        }
    }

    pub async fn tx_pool_contains(&self, tx_hash: &String) -> bool {
        self.sync_pool.contains_tx(tx_hash).await
    }

    pub async fn get_tx_pool_diff(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<String> {
        self.sync_pool.get_tx_pool_diff(tx_hashes).await
    }

    pub async fn get_txs_from_pool(
        &self,
        tx_hashes: Vec<String>,
    ) -> Vec<TxCandidate> {
        self.sync_pool.get_txs(tx_hashes).await
    }

    pub(crate) async fn make_block_candidate(
        &self,
    ) -> Result<Option<BlockCandidate>, LedgerError> {
        let tx_candidates = self.sync_pool.get_all_txs().await?;

        if tx_candidates.is_empty() {
            return Ok(None);
        }

        println!("11");

        let bc = self.consensus.do_consensus(self, tx_candidates).await?;

        println!("22");

        self.sync_pool.remove_tcs(&bc.tx_candidates).await?;

        Ok(Some(bc))
    }
}
