use crate::SakLedger;
use async_trait::async_trait;
use sak_types::{BlockCandidate, TxCandidate};

#[async_trait]
pub trait Consensus {
    async fn do_consensus(
        &self,
        sak_ledger: &SakLedger,
        txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError>;
}

pub type ConsensusResolver = Box<dyn Consensus + Send + Sync>;

pub type ConsensusError = Box<dyn std::error::Error + Send + Sync>;
