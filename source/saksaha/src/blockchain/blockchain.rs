use super::genesis;
use sak_blockchain::{Blockchain, BlockchainArgs};

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

pub(crate) struct Bchain {
    bc: Blockchain,
}

impl Bchain {
    pub async fn init(
        app_prefix: String,
        tx_pool_sync_interval: Option<u64>,
    ) -> Result<Bchain, String> {
        let genesis_block = genesis::make_genesis_block();

        let blockchain_args = BlockchainArgs {
            genesis_block,
            app_prefix,
            tx_pool_sync_interval,
        };

        let bc = Blockchain::init(blockchain_args).await?;

        Ok(Bchain { bc })
    }
}
