pub enum BlockchainEvent {
    // LedgerBlockAdd(...)
    TxPoolStat(Vec<String>),
    // TxPoolChanged(Vec<String>),
}

impl std::fmt::Display for BlockchainEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TxPoolStat(hashes) => {
                write!(f, "TxPoolStat [hashes: {:?}]", hashes)
            }
        }
    }
}
