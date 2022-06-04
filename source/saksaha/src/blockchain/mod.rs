use sak_blockchain::{Blockchain, BlockchainArgs};

pub(crate) async fn create_blockchain(
    app_prefix: String,
) -> Result<Blockchain, String> {
    let blockchain_args = BlockchainArgs { app_prefix };

    Blockchain::init(blockchain_args).await
}
