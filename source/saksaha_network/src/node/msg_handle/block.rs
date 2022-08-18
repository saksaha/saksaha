use crate::{machine::Machine, node::SaksahaNodeError};
use log::{debug, info, warn};
use sak_p2p_transport::{
    BlockAckMsg, BlockSynMsg, ErrorMsg, Msg, RecvReceipt, SendReceipt,
    UpgradedConn,
};
use sak_types::{BlockHash, BlockHeight};
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_block_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    new_blocks: Vec<(BlockHeight, BlockHash)>,
    machine: &Arc<Machine>,
) -> Result<RecvReceipt, SaksahaNodeError> {
    let block_hashes: Vec<&BlockHash> = new_blocks
        .iter()
        .map(|(_, block_hash)| block_hash)
        .collect();

    let blocks = machine
        .blockchain
        .dist_ledger
        .apis
        .get_blocks(block_hashes)
        .await?;

    let mut blocks_to_send = Vec::with_capacity(blocks.len());

    for block in blocks {
        let txs = machine
            .blockchain
            .dist_ledger
            .apis
            .get_txs(&block.tx_hashes)
            .await?;

        blocks_to_send.push((block, txs));
    }

    conn_lock
        .send(Msg::BlockSyn(BlockSynMsg {
            blocks: blocks_to_send,
        }))
        .await;

    println!("11");
    let msg_wrap = conn_lock.next_msg().await?;
    println!("22");

    let receipt = msg_wrap.get_receipt();

    let msg = msg_wrap
        .get_maybe_msg()
        .ok_or(format!("block syn needs to be followed by ack"))??;

    let _block_ack_msg = match msg {
        Msg::BlockAck(m) => m,
        Msg::Error(m) => {
            return Err(
                format!("Receiver returned error msg, msg: {:?}", m).into()
            )
        }
        _ => {
            return Err(
                format!("Only block ack should arrive at this point").into()
            );
        }
    };

    Ok(receipt)
}

pub(in crate::node) async fn recv_block_syn(
    block_syn_msg: BlockSynMsg,
    machine: &Arc<Machine>,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> SendReceipt {
    let wrapped = || async {
        println!("33");
        let blocks = block_syn_msg.blocks;

        let latest_block_height = machine
            .blockchain
            .dist_ledger
            .apis
            .get_latest_block_height()?
            .unwrap_or(0);

        for (block, txs) in blocks {
            if block.block_height != (latest_block_height + 1) {
                warn!(
                    "received not continuous block height, block_height: {}",
                    block.block_height
                );
            }

            machine
                .blockchain
                .dist_ledger
                .apis
                .sync_block(block, txs)
                .await?;
        }
        println!("44");

        let block_ack_msg = Msg::BlockAck(BlockAckMsg {});

        let receipt = conn_lock.send(block_ack_msg).await;

        Ok::<_, SaksahaNodeError>(receipt)
    };

    let receipt = match wrapped().await {
        Ok(r) => r,
        Err(err) => {
            conn_lock
                .send(Msg::Error(ErrorMsg {
                    error: err.to_string(),
                }))
                .await
        }
    };

    receipt
}
