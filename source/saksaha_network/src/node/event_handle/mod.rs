use crate::machine::Machine;
use futures::SinkExt;
use log::{info, warn};
use sak_p2p_transport::{
    BlockHashSynMsg, Msg, TxHashSynMsg, UpgradedConnection,
};
use tokio::sync::RwLockWriteGuard;

pub(super) async fn handle_tx_pool_stat<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    _machine: &Machine,
    new_tx_hashes: Vec<String>,
) {
    match conn
        .socket
        .send(Msg::TxHashSyn(TxHashSynMsg {
            tx_hashes: new_tx_hashes,
        }))
        .await
    {
        Ok(_) => {
            info!("Sending TxHashSyn, dst public_key: {}", public_key);
        }
        Err(err) => {
            warn!(
                "Failed to request to synchronize with peer node, err: {}",
                err,
            );
        }
    };
}

pub(super) async fn handle_new_blocks_ev<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    _machine: &Machine,
    new_blocks: Vec<(u128, String)>,
) {
    match conn
        .socket
        .send(Msg::BlockHashSyn(BlockHashSynMsg {
            new_blocks: new_blocks.clone(),
        }))
        .await
    {
        Ok(_) => {
            info!("Sending block hash syn, dst public_key: {}", public_key);
        }
        Err(err) => {
            warn!(
                "Failed to request to synchronize with peer node, err: {}",
                err,
            );
        }
    };
}
