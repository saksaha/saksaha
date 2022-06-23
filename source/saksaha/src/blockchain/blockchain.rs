use super::genesis;
use sak_dist_ledger::{DLedger, DLedgerArgs};
use sak_types::BlockCandidate;

// pub(crate) async fn create_blockchain(
//     app_prefix: String,
//     tx_pool_sync_interval: Option<u64>,
// ) -> Result<Blockchain, String> {
//     let genesis_block = genesis::make_genesis_block();

//     let blockchain_args = BlockchainArgs {
//         genesis_block,
//         app_prefix,
//         tx_pool_sync_interval,
//     };

//     Blockchain::init(blockchain_args).await
// }

pub(crate) struct Blockchain {
    pub(crate) dledger: DLedger,
}

impl Blockchain {
    pub async fn init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
        genesis_block: Option<BlockCandidate>,
    ) -> Result<Blockchain, String> {
        let genesis_block = match genesis_block {
            Some(b) => b,
            None => genesis::make_genesis_block(),
        };

        let dledger_args = DLedgerArgs {
            genesis_block,
            app_prefix,
            tx_pool_sync_interval,
        };

        let dledger = DLedger::init(dledger_args).await?;

        Ok(Blockchain { dledger })
    }

    pub async fn run(&self) {
        self.dledger.run().await;
    }
}
