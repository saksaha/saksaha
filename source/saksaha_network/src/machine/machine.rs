use crate::{ledger::Ledger, SaksahaError};

pub(crate) struct Machine {
    ledger: Ledger,
}

impl Machine {
    pub fn init(ledger: Ledger) -> Result<Machine, SaksahaError> {
        let m = Machine { ledger };

        Ok(m)
    }

    pub async fn run(&self) {
        tokio::join!(self.ledger.run(),);
    }
}
