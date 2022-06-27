use super::{
    genesis::GenesisBlock,
    // sys_contracts::{SystemContracts, Validator},
    Pos,
};
use crate::system::BoxedError;
use log::info;
use sak_dist_ledger::{Consensus, DistLedger, DistLedgerArgs};

pub(crate) struct Blockchain {
    pub(crate) dist_ledger: DistLedger,
    // pub(crate) sys_contracts: SystemContracts,
}

impl Blockchain {
    pub(crate) async fn init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
        genesis_block: Option<GenesisBlock>,
    ) -> Result<Blockchain, BoxedError> {
        let (gen_block_candidate, consensus) = {
            let genesis_block = match genesis_block {
                Some(b) => b,
                None => GenesisBlock::create(),
            };

            // let sys_contracts = {
            //     let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

            //     let validator = Validator::init(validator_ctr_addr);

            //     let c = SystemContracts { validator };

            //     c
            // };

            let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

            let consensus = {
                let c = Pos { validator_ctr_addr };
                Box::new(c)
            };

            (genesis_block.block_candidate, consensus)
        };

        let dist_ledger_args = DistLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
            genesis_block: Some(gen_block_candidate),
            consensus,
        };

        let dist_ledger = {
            let d = DistLedger::init(dist_ledger_args).await?;

            d
        };

        let blockchain = Blockchain {
            dist_ledger,
            // sys_contracts,
        };

        Ok(blockchain)
    }

    pub async fn run(&self) {
        self.dist_ledger.run().await;
    }

    // pub async fn get_next_validator(&self) -> Result<String, BoxedError> {
    //     self.sys_contracts
    //         .validator
    //         .get_next_validator(&self.dist_ledger)
    //         .await
    // }
}
