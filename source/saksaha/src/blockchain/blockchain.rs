use super::{
    genesis::{self, GenesisBlock},
    sys_contracts::{SystemContracts, Validator},
};
use crate::system::BoxedError;
use sak_dist_ledger::{DistLedger, DistLedgerArgs};
use sak_types::BlockCandidate;

pub(crate) struct Blockchain {
    pub(crate) dist_ledger: DistLedger,
    pub(crate) sys_contracts: SystemContracts,
}

impl Blockchain {
    pub async fn init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
        genesis_block: Option<BlockCandidate>,
    ) -> Result<Blockchain, BoxedError> {
        let genesis_block = match genesis_block {
            Some(b) => b,
            None => GenesisBlock::create(),
        };

        let dist_ledger_args = DistLedgerArgs {
            genesis_block,
            app_prefix,
            tx_pool_sync_interval,
        };

        let dist_ledger = DistLedger::init(dist_ledger_args).await?;

        let b = dist_ledger.get_gen_block().await?;
        println!("gen block inserted: {:?}", b);

        let validator_contract_addr = {
            // get addr from initial_state
            "3"
        };

        let sys_contracts = {
            let validator = Validator::init(validator_contract_addr);

            let c = SystemContracts { validator };

            c
        };

        Ok(Blockchain {
            dist_ledger,
            sys_contracts,
        })
    }

    pub async fn run(&self) {
        self.dist_ledger.run().await;
    }

    pub async fn get_next_validator(&self) -> Result<String, BoxedError> {
        self.sys_contracts.validator.get_next_validator()
    }
}
