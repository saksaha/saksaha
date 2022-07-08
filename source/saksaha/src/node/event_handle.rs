use crate::{machine::Machine, system::BoxedError};
use futures::{SinkExt, StreamExt};
use log::{info, warn};
use sak_p2p_trpt::{
    BlockHashSynMsg, BlockSynMsg, Msg, TxHashSynMsg, TxSynMsg,
    UpgradedConnection,
};
use sak_types::Block;
use std::time::Duration;
use tokio::sync::RwLockWriteGuard;

const RESPONSE_TIMEOUT: u64 = 2000;

pub(super) async fn handle_tx_pool_stat<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
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

    let resp_timeout =
        tokio::time::sleep(Duration::from_millis(RESPONSE_TIMEOUT));

    let tcs = tokio::select! {
        _ = resp_timeout => {
            warn!(
                "Peer did not respond in time, dst public_key: {}",
                public_key,
            );

            return;
        },
        resp = conn.socket.next() => {
            match resp {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => match msg {
                        Msg::TxHashAck(h) => {
                            let txs = machine
                                .blockchain
                                .dist_ledger
                                .get_txs_from_pool(h.tx_hashes)
                                .await;

                            txs
                        }
                        other_msg => {
                            // tx_hash_syn
                            warn!(
                                "Received an invalid type message, msg: {:?}",
                                other_msg,
                            );

                            return;
                        }
                    },
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                        return;
                    }
                },
                None => {
                    warn!("Received an invalid data stream");
                    return;
                }
            }
        },
    };

    if !tcs.is_empty() {
        match conn.socket.send(Msg::TxSyn(TxSynMsg { tcs })).await {
            Ok(_) => {
                info!("Sending TxSyn, public_key: {}", public_key);
            }
            Err(err) => {
                info!("Failed to send requested tx, err: {}", err,);
            }
        }
    }
}

pub(super) async fn handle_new_blocks_ev<'a>(
    public_key: &str,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
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

    let resp_timeout =
        tokio::time::sleep(Duration::from_millis(RESPONSE_TIMEOUT));

    let _peer_height = tokio::select! {
        _ = resp_timeout => {
            warn!(
                "Peer did not respond in time, dst public_key: {}",
                public_key,
            );

            return;
        },
        resp = conn.socket.next() => {
            match resp {
                Some(maybe_msg) => match maybe_msg {
                    Ok(msg) => match msg {
                        Msg::BlockHashAck(block_hash_syn_msg) => {
                            let _ = handle_block_hash_ack(block_hash_syn_msg, conn, machine).await;
                        }
                        other_msg => {
                            // tx_hash_syn
                            warn!(
                                "Received an invalid type message, msg: {:?}",
                                other_msg,
                            );

                            return;
                        }
                    },
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                        return;
                    }
                },
                None => {
                    warn!("Received an invalid data stream");
                    return;
                }
            }
        },
    };
}

async fn handle_block_hash_ack<'a>(
    block_hash_syn_msg: BlockHashSynMsg,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
) -> Result<(), BoxedError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let block_hashes: Vec<&String> = new_blocks
        .iter()
        .map(|(_, block_hash)| block_hash)
        .collect();

    // let block_candidate

    let blocks = machine
        .blockchain
        .dist_ledger
        .get_blocks(block_hashes)
        .await?;

    let mut txs_by_block = vec![vec![]];

    for block in &blocks {
        let txs = machine
            .blockchain
            .dist_ledger
            .get_txs(block.get_tx_hashes())
            .await?;

        txs_by_block.push(txs);
    }

    if !blocks.is_empty() {
        match conn
            .socket
            .send(Msg::BlockSyn(BlockSynMsg {
                blocks,
                txs: txs_by_block,
            }))
            .await
        {
            Ok(_) => {}
            Err(err) => {
                info!("Failed to send requested tx, err: {}", err,);
            }
        }
    }

    Ok(())
}
