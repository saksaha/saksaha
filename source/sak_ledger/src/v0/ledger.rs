use crate::Consensus;
use crate::LedgerDB;
use crate::LedgerError;
use crate::SyncPool;
use async_trait::async_trait;
use sak_crypto::hasher::MiMC;
use sak_crypto::MerkleTree;
use sak_ledger_cfg::CM_TREE_DEPTH;
use sak_logger::info;
use sak_store_interface::LedgerInterface;
use sak_types::{
    Block, BlockCandidate, BlockHash, BlockHeight, Cm, CmIdx, CtrAddr, CtrRequest, DistLedgerEvent,
    MintTxCandidate, PourTxCandidate, Sn, Tx, TxCandidate, TxCtrOp, TxHash,
};
use sak_vm_interface::ContractProcessor;
use std::sync::Arc;
use std::{collections::HashMap, path::PathBuf};
use tokio::sync::{
    broadcast::{self, Sender},
    Mutex,
};

const BLOCKCHAIN_EVENT_QUEUE_CAPACITY: usize = 32;

pub struct SakLedger {
    pub ledger_event_tx: Arc<Sender<DistLedgerEvent>>,
    pub ledger_db: LedgerDB,
    pub sync_pool: Arc<SyncPool>,
    pub merkle_tree: MerkleTree,
    pub hasher: MiMC,
    pub consensus: Box<dyn Consensus + Send + Sync>,
    pub contract_processor: Option<Arc<ContractProcessor>>,
    // pub contract_processor: Arc<Mutex<ContractProcessor>>,
    // pub contract_processor: Option<Box<dyn ContractProcess + Send + Sync>>,
}

pub struct SakLedgerArgs {
    pub tx_sync_interval: Option<u64>,
    pub genesis_block: Option<BlockCandidate>,
    pub consensus: Box<dyn Consensus + Send + Sync>,
    pub block_sync_interval: Option<u64>,
    pub ledger_path: PathBuf,
    pub contract_processor: Option<Arc<ContractProcessor>>,
    // pub contract_processor: Arc<Mutex<ContractProcessor>>,
}

impl SakLedger {
    pub async fn init(ledger_args: SakLedgerArgs) -> Result<SakLedger, LedgerError> {
        let SakLedgerArgs {
            tx_sync_interval,
            genesis_block,
            consensus,
            block_sync_interval,
            ledger_path,
            contract_processor,
        } = ledger_args;

        let ledger_db = LedgerDB::init(&ledger_path).await?;

        let ledger_event_tx = {
            let (tx, _rx) = broadcast::channel(BLOCKCHAIN_EVENT_QUEUE_CAPACITY);

            Arc::new(tx)
        };

        let sync_pool = {
            let tx = ledger_event_tx.clone();

            let p = SyncPool::new(tx, tx_sync_interval, block_sync_interval);

            Arc::new(p)
        };

        let hasher = MiMC::new();

        let merkle_tree = MerkleTree::new(CM_TREE_DEPTH as u32);

        let ledger = SakLedger {
            ledger_event_tx,
            ledger_db,
            sync_pool,
            merkle_tree,
            hasher,
            consensus,
            contract_processor,
        };

        if let Some(bc) = genesis_block {
            ledger.insert_genesis_block(bc).await?;
        }

        let latest_height = match ledger.ledger_db.get_latest_block_height()? {
            Some(h) => h.to_string(),
            None => "No block yet".to_string(),
        };

        info!(
            "Initialized Blockchain, latest added height (none if genesis \
            block has not been inserted): {}",
            latest_height,
        );

        Ok(ledger)
    }

    // pub fn _run(mut self, contract_processor: Box<dyn ContractProcess + Send + Sync>) {
    //     self.contract_processor.lock().await = Some(contract_processor);
    // }
}

impl SakLedger {
    pub fn _get_ledger_event_tx(&self) -> &Arc<Sender<DistLedgerEvent>> {
        &self.ledger_event_tx
    }
}

#[async_trait]
impl LedgerInterface for SakLedger {
    fn get_ledger_event_tx(&self) -> &Arc<Sender<DistLedgerEvent>> {
        self._get_ledger_event_tx()
    }

    fn get_ctr_state(&self) -> Result<Option<Vec<u8>>, LedgerError> {
        self._get_ctr_state()
    }

    async fn get_blocks(&self, block_hashes: Vec<&String>) -> Result<Vec<Block>, LedgerError> {
        self._get_blocks(block_hashes).await
    }

    async fn get_txs(&self, tx_hashes: &Vec<String>) -> Result<Vec<Tx>, LedgerError> {
        self._get_txs(tx_hashes).await
    }

    async fn get_merkle_node(&self, location: &String) -> Result<[u8; 32], LedgerError> {
        self._get_merkle_node(location).await
    }

    async fn get_auth_path(&self, cm_idx: &u128) -> Result<Vec<([u8; 32], bool)>, LedgerError> {
        self._get_auth_path(cm_idx).await
    }

    async fn get_cm_idx_by_cm(&self, cm: &Cm) -> Result<Option<CmIdx>, LedgerError> {
        self._get_cm_idx_by_cm(cm).await
    }

    async fn get_latest_block_hash(&self) -> Result<Option<(BlockHeight, BlockHash)>, LedgerError> {
        self._get_latest_block_hash().await
    }

    async fn send_tx(&self, tx_candidate: TxCandidate) -> Result<TxHash, LedgerError> {
        self._send_tx(tx_candidate).await
    }

    async fn get_tx(&self, tx_hash: &String) -> Result<Option<Tx>, LedgerError> {
        self._get_tx(tx_hash).await
    }

    fn get_block(&self, block_hash: &String) -> Result<Option<Block>, LedgerError> {
        self._get_block(block_hash)
    }

    async fn get_block_list(
        &self,
        offset: Option<u128>,
        limit: Option<u128>,
    ) -> Result<Vec<Block>, LedgerError> {
        self._get_block_list(offset, limit).await
    }

    async fn get_all_blocks(&self) -> Result<Vec<(BlockHeight, BlockHash)>, LedgerError> {
        self._get_all_blocks().await
    }

    async fn get_block_by_height(&self, block_height: &u128) -> Result<Option<Block>, LedgerError> {
        self._get_block_by_height(block_height).await
    }

    fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerError> {
        self._get_latest_block_height()
    }

    async fn get_latest_block_merkle_rt(&self) -> Result<Option<[u8; 32]>, LedgerError> {
        self._get_latest_block_merkle_rt().await
    }

    async fn insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<Option<String>, LedgerError> {
        self._insert_genesis_block(genesis_block).await
    }

    async fn write_block(&self, bc: Option<BlockCandidate>) -> Result<Option<String>, LedgerError> {
        self._write_block(bc).await
    }

    async fn write_blocks(
        &self,
        mut blocks: Vec<(Block, Vec<Tx>)>,
    ) -> Result<Vec<String>, LedgerError> {
        self._write_blocks(blocks).await
    }

    fn verify_merkle_rt(&self, merkle_rt: &[u8; 32]) -> bool {
        self._verify_merkle_rt(merkle_rt)
    }

    fn verify_sn(&self, sn: &Sn) -> Result<bool, LedgerError> {
        self._verify_sn(sn)
    }

    fn verify_proof(&self, tc: &PourTxCandidate) -> Result<bool, LedgerError> {
        self._verify_proof(tc)
    }

    fn filter_tx_candidates(&self, bc: &mut BlockCandidate) -> Result<(), LedgerError> {
        self._filter_tx_candidates(bc)
    }

    async fn process_ctr_state_update(
        &self,
        ctr_addr: &String,
        data: &[u8],
        tx_ctr_op: TxCtrOp,
        ctr_state_update: &mut HashMap<CtrAddr, Vec<u8>>,
    ) -> Result<(), LedgerError> {
        self._process_ctr_state_update(ctr_addr, data, tx_ctr_op, ctr_state_update)
            .await
    }

    async fn handle_mint_tx_candidate(
        &self,
        tc: &MintTxCandidate,
        ctr_state_update: &mut HashMap<CtrAddr, Vec<u8>>,
        merkle_update: &mut HashMap<String, [u8; 32]>,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        self._handle_mint_tx_candidate(tc, ctr_state_update, merkle_update, next_cm_idx)
            .await
    }

    async fn handle_pour_tx_candidate(
        &self,
        tc: &PourTxCandidate,
        ctr_state_update: &mut HashMap<CtrAddr, Vec<u8>>,
        merkle_update: &mut HashMap<String, [u8; 32]>,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        self._handle_pour_tx_candidate(tc, ctr_state_update, merkle_update, next_cm_idx)
            .await
    }

    async fn process_merkle_update(
        &self,
        merkle_update: &mut HashMap<String, [u8; 32]>,
        cms: &Vec<[u8; 32]>,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerError> {
        self._process_merkle_update(merkle_update, cms, next_cm_idx)
            .await
    }

    async fn execute_ctr(&self, req: CtrRequest) -> Result<Vec<u8>, LedgerError> {
        self._execute_ctr(req).await
    }

    async fn update_ctr(&self, req: CtrRequest) -> Result<Vec<u8>, LedgerError> {
        self._update_ctr(req).await
    }

    async fn insert_into_pool(&self, tx_candidates: Vec<TxCandidate>) {
        self._insert_into_pool(tx_candidates).await
    }

    async fn tx_pool_contains(&self, tx_hash: &String) -> bool {
        self._tx_pool_contains(tx_hash).await
    }

    async fn get_tx_pool_diff(&self, tx_hashes: Vec<String>) -> Vec<String> {
        self._get_tx_pool_diff(tx_hashes).await
    }

    async fn get_txs_from_pool(&self, tx_hashes: Vec<String>) -> Vec<TxCandidate> {
        self._get_txs_from_pool(tx_hashes).await
    }

    async fn make_block_candidate(&self) -> Result<Option<BlockCandidate>, LedgerError> {
        self._make_block_candidate().await
    }
}
