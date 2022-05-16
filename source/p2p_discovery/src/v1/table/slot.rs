use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub(crate) struct Slot {
    idx: usize,
}

pub(crate) struct SlotGuard {
    slot: Arc<Slot>,
    slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
}
