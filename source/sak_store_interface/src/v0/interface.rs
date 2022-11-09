use async_trait::async_trait;
use sak_types::{
    Block, BlockCandidate, BlockHash, BlockHeight, Cm, CmIdx, CtrAddr, MintTxCandidate,
    PourTxCandidate, Sn, Tx, TxCandidate, TxCtrOp, TxHash,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type MRSInterfaceError = Box<dyn std::error::Error + Send + Sync>;

pub type LedgerInterfaceError = Box<dyn std::error::Error + Send + Sync>;

pub type MRSAccessor = Box<dyn MRSInterface + Send + Sync>;

pub type LedgerAccessor = Box<dyn LedgerInterface + Send + Sync>;

pub trait MRSInterface {
    fn get_mrs_data(&self, key: &String) -> Result<Option<String>, MRSInterfaceError>;

    fn put_mrs_data(&self, key: &String, value: &String) -> Result<(), MRSInterfaceError>;

    // async fn get_session(&self, session_id: String) -> Session;
    fn add_session(&self, session: Session);
}

// TEMP
type Storage = Vec<u8>;

type CtrStateUpdate = HashMap<CtrAddr, Storage>;

type MerkleUpdate = HashMap<MerkleNodeLoc, [u8; 32]>;

type MerkleNodeLoc = String;
// TEMP

#[async_trait]
pub trait LedgerInterface {
    fn get_ctr_state(&self) -> Result<Option<Vec<u8>>, LedgerInterfaceError>;
    async fn get_blocks(
        &self,
        block_hashes: Vec<&String>,
    ) -> Result<Vec<Block>, LedgerInterfaceError>;
    async fn get_txs(&self, tx_hashes: &Vec<String>) -> Result<Vec<Tx>, LedgerInterfaceError>;
    async fn get_merkle_node(&self, location: &String) -> Result<[u8; 32], LedgerInterfaceError>;
    async fn get_auth_path(
        &self,
        cm_idx: &u128,
    ) -> Result<Vec<([u8; 32], bool)>, LedgerInterfaceError>;
    async fn get_cm_idx_by_cm(&self, cm: &Cm) -> Result<Option<CmIdx>, LedgerInterfaceError>;
    async fn get_latest_block_hash(
    ) -> Result<Option<(BlockHeight, BlockHash)>, LedgerInterfaceError>;
    async fn send_tx(&self, tx_candidate: TxCandidate) -> Result<TxHash, LedgerInterfaceError>;
    async fn get_tx(&self, tx_hash: &String) -> Result<Option<Tx>, LedgerInterfaceError>;
    fn get_block(&self, block_hash: &String) -> Result<Option<Block>, LedgerInterfaceError>;
    async fn get_block_list(
        &self,
        offset: Option<u128>,
        limit: Option<u128>,
    ) -> Result<Vec<Block>, LedgerInterfaceError>;
    async fn get_all_blocks(&self) -> Result<Vec<(BlockHeight, BlockHash)>, LedgerInterfaceError>;
    async fn get_block_by_height(
        &self,
        block_height: &u128,
    ) -> Result<Option<Block>, LedgerInterfaceError>;
    fn get_latest_block_height(&self) -> Result<Option<u128>, LedgerInterfaceError>;
    async fn get_latest_block_merkle_rt(&self) -> Result<Option<[u8; 32]>, LedgerInterfaceError>;
    async fn insert_genesis_block(
        &self,
        genesis_block: BlockCandidate,
    ) -> Result<Option<String>, LedgerInterfaceError>;
    async fn write_block(
        &self,
        bc: Option<BlockCandidate>,
    ) -> Result<Option<String>, LedgerInterfaceError>;
    async fn write_blocks(
        &self,
        mut blocks: Vec<(Block, Vec<Tx>)>,
    ) -> Result<Vec<String>, LedgerInterfaceError>;
    fn verify_merkle_rt(&self, merkle_rt: &[u8; 32]) -> bool;
    fn verify_sn(&self, sn: &Sn) -> Result<bool, LedgerInterfaceError>;
    fn verify_proof(&self, tc: &PourTxCandidate) -> Result<bool, LedgerInterfaceError>;
    fn filter_tx_candidates(&self, bc: &mut BlockCandidate) -> Result<(), LedgerInterfaceError>;
    async fn process_ctr_state_update(
        &self,
        ctr_addr: &String,
        data: &[u8],
        tx_ctr_op: TxCtrOp,
        ctr_state_update: &mut CtrStateUpdate,
    ) -> Result<(), LedgerInterfaceError>;
    async fn handle_mint_tx_candidate(
        &self,
        tc: &MintTxCandidate,
        ctr_state_update: &mut CtrStateUpdate,
        merkle_update: &mut MerkleUpdate,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerInterfaceError>;
    async fn handle_pour_tx_candidate(
        &self,
        tc: &PourTxCandidate,
        ctr_state_update: &mut CtrStateUpdate,
        merkle_update: &mut MerkleUpdate,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerInterfaceError>;
    async fn process_merkle_update(
        &self,
        merkle_update: &mut MerkleUpdate,
        cms: &Vec<[u8; 32]>,
        next_cm_idx: CmIdx,
    ) -> Result<u128, LedgerInterfaceError>;
}

#[derive(Serialize, Deserialize)]
pub struct PreflightResponse {
    pub request_id: usize,
    pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub receipt: HashMap<String, Vec<u8>>,
}
