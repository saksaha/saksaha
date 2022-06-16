use super::genesis;
use sak_blockchain::{Blockchain, BlockchainArgs};

pub(crate) async fn create_blockchain(
    app_prefix: String,
    tx_pool_sync_interval: Option<u64>,
) -> Result<Blockchain, String> {
    let genesis_block = genesis::make_genesis_block();

    let blockchain_args = BlockchainArgs {
        genesis_block,
        app_prefix,
        tx_pool_sync_interval,
    };

    Blockchain::init(blockchain_args).await
}
