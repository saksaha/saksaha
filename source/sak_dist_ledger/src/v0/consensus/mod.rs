use crate::DistLedger;
use std::{future::Future, pin::Pin};

pub trait Consensus<C> {
    fn do_consensus(
        &self,
        dist_ledger: DistLedger<C>,
    ) -> Pin<Box<dyn Future<Output = Result<(), String>> + Send + 'static>>;
}
