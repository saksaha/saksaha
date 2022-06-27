use crate::machine::Machine;
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

    let txs = tokio::select! {
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

    if !txs.is_empty() {
        match conn.socket.send(Msg::TxSyn(TxSynMsg { txs })).await {
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
    new_blocks: Vec<(String, String)>,
) {
    println!("sending block hash syn msg!!");

    match conn
        .socket
        .send(Msg::BlockHashSyn(BlockHashSynMsg {
            new_blocks: new_blocks.clone(),
        }))
        .await
    {
        Ok(_) => {
            info!("Sending HeightSyn, dst public_key: {}", public_key);
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
                            handle_block_hash_ack(block_hash_syn_msg, conn, machine);
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

    // let peer_height = peer_height.parse::<usize>().unwrap() + 1;
    // let new_height = new_height.parse::<usize>().unwrap() + 1;
    // let block_height_vec =
    //     (peer_height..new_height as usize).collect::<Vec<_>>();

    // let mut blocks: Vec<Block> =
    //     Vec::with_capacity(new_height - peer_height as usize);

    // for idx in peer_height..new_height {
    //     let block_height = idx.to_string();

    //     let block = match machine
    //         .blockchain
    //         .dist_ledger
    //         .get_block_by_height(&block_height)
    //         .await
    //     {
    //         Ok(b) => b.unwrap(),
    //         Err(err) => {
    //             warn!(" ****** There is a probability that an error will occur, but not return error, {} ", err);

    //             return;
    //         }
    //     };

    //     blocks.push(block)
    // }

    // if !block_height_vec.is_empty() {
    //     match conn.socket.send(Msg::BlockSyn(BlockSyn { blocks })).await {
    //         Ok(_) => {
    //             info!("Sending BlockSyn, public_key: {}", public_key);
    //         }
    //         Err(err) => {
    //             info!("Failed to send requested tx, err: {}", err,);
    //         }
    //     }
    // }
}

async fn handle_block_hash_ack<'a>(
    block_hash_syn_msg: BlockHashSynMsg,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
    machine: &Machine,
) {
    let new_blocks = block_hash_syn_msg.new_blocks;

    for (_, block_hash) in new_blocks {
        // machine.blockchain.dist_leger.get_
    }
}
