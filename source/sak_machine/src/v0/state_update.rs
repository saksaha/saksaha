use sak_contract_std::Storage;
use sak_types::CtrAddr;
use std::collections::HashMap;

pub(crate) type CtrStateUpdate = HashMap<CtrAddr, Storage>;

pub(crate) type MerkleUpdate = HashMap<MerkleNodeLoc, [u8; 32]>;

pub(crate) type MerkleNodeLoc = String;
