use crate::DistLedger;
use sak_types::{BlockCandidate, Tx};
use std::{future::Future, pin::Pin, sync::Arc};

pub trait Consensus {
    fn do_consensus(
        &self,
        dist_ledger: &DistLedger,
        txs: Vec<Tx>,
    ) -> Pin<
        Box<
            dyn Future<Output = Result<BlockCandidate, String>>
                + Send
                + 'static,
        >,
    >;
}
