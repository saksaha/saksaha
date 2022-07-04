use crate::{machine::Machine, system::BoxedError};
use futures::{SinkExt, StreamExt};
use log::{debug, info, warn};
use sak_p2p_trpt::{
    BlockHashSynMsg, BlockSynMsg, Msg, TxHashSynMsg, UpgradedConnection,
};
use std::time::Duration;
use tokio::sync::RwLockWriteGuard;

const RESPONSE_TIMEOUT: u64 = 1000;

pub(crate) async fn handle_msg<'a>(
    msg: Msg,
    public_key: &str,
    machine: &Machine,
    mut conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
) -> Result<(), BoxedError> {
    match msg {
        Msg::TxHashSyn(tx_hash_syn_msg) => {
            handle_tx_hash_syn(public_key, tx_hash_syn_msg, machine, &mut conn)
                .await
        }
        Msg::BlockHashSyn(block_hash_syn_msg) => {
            handle_block_hash_syn(block_hash_syn_msg, machine, &mut conn)
                .await?;
        }
        Msg::BlockSyn(block_syn_msg) => {
            handle_block_syn(block_syn_msg, machine, &mut conn).await?;
        }
        // Msg::BlockHeightSyn(block_height_syn_msg) => {
        //     handle_block_height_syn(block_height_syn_msg, machine, &mut conn)
        //         .await?;
        // }
        _ => (),
    };

    Ok(())
}

pub(crate) async fn handle_tx_hash_syn<'a>(
    public_key: &str,
    tx_hash_syn_msg: TxHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
) {
    let txs_to_request = machine
        .blockchain
        .dist_ledger
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    match conn
        .socket
        .send(Msg::TxHashAck(TxHashSynMsg {
            tx_hashes: txs_to_request,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to send requested tx, err: {}", err,);
        }
    };

    let resp_timeout =
        tokio::time::sleep(Duration::from_millis(RESPONSE_TIMEOUT));

    let _txs = tokio::select! {
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
                        Msg::TxSyn(h) => {
                            info!(
                                "Handling TxSyn msg, src public_key: {}",
                                public_key
                            );

                            machine.blockchain.dist_ledger
                                .insert_into_pool(h.txs).await;
                        }
                        other_msg => {
                            warn!(
                                "Received an invalid type message, msg: {:?}",
                                other_msg,
                            );
                        }
                    },
                    Err(err) => {
                        warn!("Failed to parse the msg, err: {}", err);
                    }
                },
                None => {
                    warn!("Received an invalid data stream");
                }
            };
        }
    };
}

pub(crate) async fn handle_block_hash_syn<'a>(
    block_hash_syn_msg: BlockHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
) -> Result<(), BoxedError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let (_, latest_block_hash) = machine
        .blockchain
        .dist_ledger
        .get_latest_block_hash()
        .await?
        .ok_or("height does not exist")?;

    debug!(
        "handle block hash syn, latest_block_hash: {}, received_new_blocks: {:?}",
        latest_block_hash,
        new_blocks,
    );

    let mut blocks_to_req = vec![];
    for (height, block_hash) in new_blocks {
        if block_hash != latest_block_hash {
            blocks_to_req.push((height, block_hash));
        }
    }

    match conn
        .socket
        .send(Msg::BlockHashAck(BlockHashSynMsg {
            new_blocks: blocks_to_req,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to send requested tx, err: {}", err,);
        }
    };

    Ok(())
}

pub(crate) async fn handle_block_syn<'a>(
    block_syn_msg: BlockSynMsg,
    machine: &Machine,
    _conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
) -> Result<(), BoxedError> {
    let block_candidates = block_syn_msg.block_candidates;

    for bc in block_candidates {
        machine.blockchain.dist_ledger.write_block(Some(bc)).await?;
    }

    Ok(())
}

// pub(crate) async fn handle_block_height_syn<'a>(
//     block_height_syn_msg: BlockHeightSynMsg,
//     machine: &Machine,
//     conn: &'a mut RwLockWriteGuard<'_, UpgradedConnection>,
// ) -> Result<(), BoxedError> {
//     let latest_height = machine
//         .blockchain
//         .dist_ledger
//         .get_latest_block_height()
//         .await?
//         .unwrap_or(0);

//     conn.socket
//         .send(Msg::BlockHeightAck(BlockHeightSynMsg {
//             block_height: latest_height,
//         }))
//         .await?;

//     println!("ack send ok");

//     Ok(())
// }
