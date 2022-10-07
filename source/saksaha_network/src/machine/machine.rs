use crate::ledger::Ledger;

pub(crate) struct Machine {
    pub(crate) ledger: Ledger,
}

impl Machine {
    pub async fn run(&self) {
        tokio::join!(self.ledger.run(),);
    }
}
