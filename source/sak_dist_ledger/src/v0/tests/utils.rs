use crate::{
    Consensus, ConsensusError, DistLedger, DistLedgerApis, DistLedgerArgs,
};
use async_trait::async_trait;
use sak_contract_std::{CtrCallType, CtrRequest};
use sak_crypto::{rand, MerkleTree, Scalar, ScalarExt};
use sak_dist_ledger_meta::CM_TREE_DEPTH;
use sak_proofs::{CoinProof, Hasher, NewCoin, OldCoin};
use sak_types::{
    BlockCandidate, PourTx, PourTxCandidate, Tx, TxCandidate, WASM_MAGIC_NUMBER,
};

pub struct DummyPos {}

#[async_trait]
impl Consensus for DummyPos {
    async fn do_consensus(
        &self,
        _dist_ledger_apis: &DistLedgerApis,
        _txs: Vec<TxCandidate>,
    ) -> Result<BlockCandidate, ConsensusError> {
        return Err("awel".into());
    }
}

pub(crate) async fn make_dist_ledger() -> DistLedger {
    let pos = make_dummy_pos();

    let dist_ledger_args = DistLedgerArgs {
        app_prefix: String::from("test"),
        tx_sync_interval: None,
        genesis_block: Some(sak_types::mock_block_1()),
        consensus: pos,
        block_sync_interval: None,
    };

    let dist_ledger = DistLedger::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub(crate) fn make_dummy_pos() -> Box<DummyPos> {
    Box::new(DummyPos {})
}
