use super::{genesis::GenesisBlock, Pos};
use crate::system::BoxedError;
use sak_dist_ledger::{Consensus, DistLedger, DistLedgerArgs};
use sak_p2p_id::Identity;
use sak_proofs::{coin_ownership::CoinProof, MiMC};
use std::sync::Arc;

pub(crate) struct Blockchain {
    pub(crate) dist_ledger: DistLedger,
}

impl Blockchain {
    pub(crate) async fn init(
        app_prefix: String,
        tx_sync_interval: Option<u64>,
        genesis_block: Option<GenesisBlock>,
        block_sync_interval: Option<u64>,
        identity: Arc<Identity>,
    ) -> Result<Blockchain, BoxedError> {
        let (gen_block_candidate, consensus) = {
            let genesis_block = match genesis_block {
                Some(b) => b,
                None => GenesisBlock::create(),
            };

            let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

            let consensus: Box<dyn Consensus + Send + Sync> = {
                let c = Pos {
                    validator_ctr_addr,
                    identity,
                };

                Box::new(c)
            };

            (genesis_block.block_candidate, consensus)
        };

        let dist_ledger_args = DistLedgerArgs {
            app_prefix,
            tx_sync_interval,
            genesis_block: Some(gen_block_candidate),
            consensus,
            block_sync_interval,
        };

        let dist_ledger = {
            let d = DistLedger::init(dist_ledger_args).await?;

            d
        };

        let blockchain = Blockchain { dist_ledger };

        Ok(blockchain)
    }

    pub async fn run(&self) {
        self.dist_ledger.run().await;
    }
}
