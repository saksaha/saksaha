use sak_types::CtrAddr;
use std::collections::HashMap;

pub(crate) type CtrStateUpdate = HashMap<CtrAddr, String>;

pub(crate) type MerkleUpdate = HashMap<MerkleNodeLoc, [u8; 32]>;

pub(crate) type MerkleNodeLoc = String;
