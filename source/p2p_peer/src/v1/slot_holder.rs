use logger::{terr, tinfo};
use std::sync::Arc;
use tokio::sync::{
    mpsc::{self, Receiver, Sender, UnboundedReceiver, UnboundedSender},
    OwnedRwLockMappedWriteGuard, OwnedRwLockWriteGuard, RwLock,
};

pub struct SlotHolder {
    pub idx: usize,
}

pub struct SlotHolderGuard {
    pub slot_holder: Arc<SlotHolder>,
    pub slots_tx: Arc<UnboundedSender<Arc<SlotHolder>>>,
}

impl Drop for SlotHolderGuard {
    fn drop(&mut self) {
        self.slots_tx.send(self.slot_holder.clone());
    }
}

// impl Drop for SlotHolder {
//     fn drop(&mut self) {
//         let to_drop = std::mem::replace(self, SlotHolder { idx: self.idx });
//         to_drop.dropping_function();
//     }
// }
