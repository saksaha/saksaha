mod block;
mod block_update;
mod contract;
mod pool;

use crate::{Consensus, LedgerDB, SyncPool};
use sak_crypto::MerkleTree;
use sak_proof::MiMC;
use sak_vm::VM;
use std::sync::Arc;

pub struct DistLedgerApis {
    pub(crate) ledger_db: LedgerDB,
    pub vm: VM,
    pub(crate) sync_pool: Arc<SyncPool>,
    pub merkle_tree: MerkleTree,
    pub hasher: MiMC,
    pub(crate) consensus: Box<dyn Consensus + Send + Sync>,
}
