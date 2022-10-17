use crate::{ledger::Ledger, SaksahaError};
use sak_ledger::SakLedger;

pub(crate) struct Machine {
    pub ledger: SakLedger,
}

impl Machine {
    pub fn init(ledger: SakLedger) -> Result<Machine, SaksahaError> {
        let m = Machine { ledger };

        Ok(m)
    }

    pub async fn run(&self) {
        tokio::join!(self.ledger.run(),);
    }
}
