use super::{
    genesis::GenesisBlock,
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
    pub(crate) async fn init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
    ) -> Result<Blockchain, BoxedError> {
        let genesis_block = GenesisBlock::create();

        Self::_init(app_prefix, tx_pool_sync_interval, genesis_block).await
    }

    async fn _init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
        genesis_block: BlockCandidate,
    ) -> Result<Blockchain, BoxedError> {
        let dist_ledger_args = DistLedgerArgs {
            // genesis_block,
            app_prefix,
            tx_pool_sync_interval,
        };

        let dist_ledger = DistLedger::init(dist_ledger_args).await?;

        {
            dist_ledger.write_block(genesis_block).await;

            let gen_block = dist_ledger.get_gen_block().await?;
            // check_if_gen_block_is_identitcal(gen_block, genesis_block)?
        }

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

// fn check_if_gen_block_is_identitcal(inserted_gen_block: Block, gen_block: BlockCandidate)

mod testing {
    use super::Blockchain;
    use super::{BlockCandidate, GenesisBlock};

    impl Blockchain {
        pub async fn test_init(
            app_prefix: String,
            tx_pool_sync_interval: Option<u64>,
            genesis_block: Option<BlockCandidate>,
        ) {
            let gen_block = match genesis_block {
                Some(b) => b,
                None => GenesisBlock::create(),
            };

            Self::_init(app_prefix, tx_pool_sync_interval, gen_block).await;
        }
    }
}
