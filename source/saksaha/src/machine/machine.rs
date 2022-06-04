use sak_blockchain::Blockchain;

pub(crate) struct Machine {
    pub(crate) blockchain: Blockchain,
}

impl Machine {
    pub async fn run(&self) {
        tokio::join!(self.blockchain.run(),);
    }
}
