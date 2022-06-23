use super::{
    genesis::{self, GenesisBlock},
    sys_contracts::{SystemContracts, Validator},
};
use crate::system::BoxedError;
use sak_dist_ledger::{DLedger, DLedgerArgs};
use sak_types::BlockCandidate;

pub(crate) struct Blockchain {
    pub(crate) dledger: DLedger,
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

        let dledger_args = DLedgerArgs {
            genesis_block,
            app_prefix,
            tx_pool_sync_interval,
        };

        let (dledger, initial_state) = DLedger::init(dledger_args).await?;

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
            dledger,
            sys_contracts,
        })
    }

    pub async fn run(&self) {
        self.dledger.run().await;
    }

    pub async fn get_next_validator(&self) -> Result<String, BoxedError> {
        self.sys_contracts.validator.get_next_validator()
    }
}
