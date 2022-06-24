use super::{
    genesis::GenesisBlock,
    sys_contracts::{SystemContracts, Validator},
};
use crate::system::BoxedError;
use log::info;
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
        genesis_block: GenesisBlock,
    ) -> Result<Blockchain, BoxedError> {
        let dist_ledger_args = DistLedgerArgs {
            app_prefix,
            tx_pool_sync_interval,
        };

        let dist_ledger = {
            let d = DistLedger::init(dist_ledger_args).await?;

            let gen_block_candidate = &genesis_block.block_candidate;

            insert_genesis_block(&d, gen_block_candidate).await?;

            d
        };

        let sys_contracts = {
            let validator_ctr_addr = genesis_block.get_validator_ctr_addr();

            let validator = Validator::init(validator_ctr_addr);

            let c = SystemContracts { validator };

            c
        };

        let blockchain = Blockchain {
            dist_ledger,
            sys_contracts,
        };

        Ok(blockchain)
    }

    pub async fn run(&self) {
        self.dist_ledger.run().await;
    }

    pub async fn get_next_validator(&self) -> Result<String, BoxedError> {
        println!("getting next validator!!");

        self.sys_contracts
            .validator
            .get_next_validator(&self.dist_ledger)
            .await
    }
}

async fn insert_genesis_block(
    dist_ledger: &DistLedger,
    genesis_block: &BlockCandidate,
) -> Result<String, String> {
    let persisted_gen_block_hash = if let Some(b) =
        dist_ledger.get_block_by_height(String::from("0")).await?
    {
        let block_hash = b.get_hash().to_string();

        info!(
            "Genesis block is already persisted, block_hash: {}",
            block_hash,
        );

        block_hash
    } else {
        let b = dist_ledger.write_block(&genesis_block).await?;

        info!("Wrote genesis block, block_hash: {}", &b);

        b
    };

    let (gen_block, _) = genesis_block.extract();
    let gen_block_hash = gen_block.get_hash();

    if gen_block_hash != &persisted_gen_block_hash {
        return Err(format!(
            "Not identical genesis block. Hardwird genesis \
            block may have been tampered, gen_block: {}, persisted: {}",
            &gen_block_hash, &persisted_gen_block_hash,
        )
        .into());
    }

    Ok(persisted_gen_block_hash.to_string())
}

mod testing {
    use super::*;

    impl Blockchain {
        pub async fn _test_init(
            app_prefix: String,
            tx_pool_sync_interval: Option<u64>,
            genesis_block: Option<GenesisBlock>,
        ) -> Result<Blockchain, BoxedError> {
            let gen_block = match genesis_block {
                Some(b) => b,
                None => GenesisBlock::create(),
            };

            Self::_init(app_prefix, tx_pool_sync_interval, gen_block).await
        }
    }
}
