use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) struct Slot {
    pub(crate) _idx: usize,
}

pub(crate) struct SlotGuard {
    pub(crate) slot: Arc<Slot>,
    pub(crate) slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
}
