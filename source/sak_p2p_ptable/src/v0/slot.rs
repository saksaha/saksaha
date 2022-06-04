use sak_logger::terr;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;

pub struct Slot {
    pub idx: usize,
}

pub struct SlotGuard {
    pub slot: Arc<Slot>,
    pub slots_tx: Arc<UnboundedSender<Arc<Slot>>>,
}

impl Drop for SlotGuard {
    fn drop(&mut self) {
        match self.slots_tx.send(self.slot.clone()) {
            Ok(_) => (),
            Err(err) => {
                terr!(
                    "p2p_peer_table",
                    "slot",
                    "Cannot send the released slot back to the queue,
                    err: {}",
                    err,
                );
            }
        }
    }
}
