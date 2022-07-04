#[derive(Clone, Debug)]
pub enum DistLedgerEvent {
    TxPoolStat(Vec<String>),
    NewBlocks(Vec<(u128, String)>),
}

impl std::fmt::Display for DistLedgerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TxPoolStat(hashes) => {
                write!(f, "TxPoolStat [hashes: {:?}]", hashes)
            }
            Self::NewBlocks(new_blocks) => {
                write!(f, "NewBlocks [len: {}]", new_blocks.len())
            }
        }
    }
}
