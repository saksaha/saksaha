use super::DistLedger;
use sak_types::Tx;

impl DistLedger {
    pub fn is_valid_tx(&self, _tx: &Tx) -> bool {
        // TODO
        true
    }
}
