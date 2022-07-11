use std::collections::HashMap;

pub(crate) type CtrStateUpdate = HashMap<String, String>;

pub(crate) type MerkleUpdate = HashMap<String, Vec<u8>>;
