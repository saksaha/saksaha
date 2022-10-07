use crate::{Consensus, ConsensusError, SakMachine};
use async_trait::async_trait;
use sak_types::{BlockCandidate, TxCandidate};

pub struct MockPos {}

#[async_trait]
impl Consensus for MockPos {
    async fn do_consensus(
        &self,
        dist_ledger: &SakMachine,
        _txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}

pub(crate) fn mock_pos() -> Box<MockPos> {
    Box::new(MockPos {})
}
