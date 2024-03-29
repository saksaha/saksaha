use sak_logger::error;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub struct Slot {
    pub idx: isize,
}

pub struct SlotGuard {
    pub slot: Slot,
    pub slots_tx: Arc<UnboundedSender<Slot>>,
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        let slot = std::mem::replace(&mut self.slot, Slot { idx: -1 });

        match self.slots_tx.send(slot) {
            Ok(_) => (),
            Err(err) => {
                error!(
                    "Cannot send the released slot back to the queue,\
                    err: {}",
                    err,
                );
            }
        }
    }
}
