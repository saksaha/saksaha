use sak_blockchain::{Blockchain, BlockchainArgs};

pub(crate) async fn create_blockchain(
    app_prefix: String,
    tx_pool_sync_interval: Option<u64>,
) -> Result<Blockchain, String> {
    let blockchain_args = BlockchainArgs {
        app_prefix,
        tx_pool_sync_interval,
    };

    Blockchain::init(blockchain_args).await
}
