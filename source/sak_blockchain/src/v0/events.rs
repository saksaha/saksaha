pub enum BlockchainEvent {
    // LedgerBlockAdd(...)
    TxPoolStat(Vec<String>),
    // TxPoolChanged(Vec<String>),
}
