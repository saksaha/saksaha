use crate::DistLedger;
use async_trait::async_trait;
use sak_types::{BlockCandidate, Tx};
use std::{future::Future, pin::Pin};

#[async_trait]
pub trait Consensus {
    async fn do_consensus(
        &self,
        dist_ledger: &DistLedger,
        txs: Vec<Tx>,
    ) -> Result<BlockCandidate, ConsensusError>;
}

pub type ConsensusError = Box<dyn std::error::Error + Send + Sync>;
