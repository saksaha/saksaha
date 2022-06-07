use crate::Transaction;

pub enum BlockchainEvent {
    // LedgerBlockAdd(...)
    TxPoolStat(Vec<Transaction>),
    TxPoolChanged(Vec<String>),
}
