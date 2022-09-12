use crate::{mock_pos, DistLedger, DistLedgerArgs};
use sak_types::BlockCandidate;

pub async fn mock_dist_ledger(block: BlockCandidate) -> DistLedger {
    let pos = mock_pos();

    let dist_ledger_args = DistLedgerArgs {
        // app_prefix: String::from("test"),
        public_key: String::from("test"),
        tx_sync_interval: None,
        genesis_block: Some(block),
        consensus: pos,
        block_sync_interval: None,
    };

    let dist_ledger = DistLedger::init(dist_ledger_args)
        .await
        .expect("Blockchain should be initialized");

    dist_ledger
}

pub async fn mock_dist_ledger_1() -> DistLedger {
    let pos = mock_pos();

    let dist_ledger_args = DistLedgerArgs {
        // app_prefix: String::from("test"),
        public_key: String::from("test"),
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
