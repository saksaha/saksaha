use crate::DistLedger;
use sak_types::{BlockCandidate, Tx};
use std::{future::Future, pin::Pin, sync::Arc};

pub trait Consensus {
    fn do_consensus<'a>(
        self: &'a Self,
        dist_ledger: &'a DistLedger,
        txs: Vec<Tx>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<BlockCandidate, ConsensusError>>
                + Send
                + 'a,
        >,
    >
    where
        Self: 'a;
}

pub type ConsensusError = Box<dyn std::error::Error + Send + Sync>;
