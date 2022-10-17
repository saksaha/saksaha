use crate::node::SaksahaNodeError;
use sak_machine::SakMachine;
use sak_p2p_transport::{BlockAckMsg, BlockSynMsg, ErrorMsg, Msg, UpgradedConn};
use sak_types::{BlockHash, BlockHeight};
use std::sync::Arc;
use tokio::sync::RwLockWriteGuard;

pub(in crate::node) async fn send_block_syn(
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
    new_blocks: Vec<(BlockHeight, BlockHash)>,
    machine: &Arc<SakMachine>,
) -> Result<(), SaksahaNodeError> {
    let block_hashes: Vec<&BlockHash> = new_blocks
        .iter()
        .map(|(_, block_hash)| block_hash)
        .collect();

    let blocks = machine
        .ledger
        // .dist_ledger
        .get_blocks(block_hashes)
        .await?;

    let mut blocks_to_send = Vec::with_capacity(blocks.len());

    for block in blocks {
        let txs = machine
            .ledger
            // .dist_ledger
            .get_txs(&block.tx_hashes)
            .await?;

        blocks_to_send.push((block, txs));
    }

    conn_lock
        .send(Msg::BlockSyn(BlockSynMsg {
            blocks: blocks_to_send,
        }))
        .await;

    Ok(())
}

pub(in crate::node) async fn recv_block_ack(
    block_ack_msg: BlockAckMsg,
    machine: &Arc<SakMachine>,
) -> Result<(), SaksahaNodeError> {
    Ok(())
}

pub(in crate::node) async fn recv_block_syn(
    block_syn_msg: BlockSynMsg,
    machine: &Arc<SakMachine>,
    mut conn_lock: RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaNodeError> {
    let blocks = block_syn_msg.blocks;

    let _ = machine
        .ledger
        // .dist_ledger
        .write_blocks(blocks)
        .await;

    let block_ack_msg = Msg::BlockAck(BlockAckMsg {});

    conn_lock.send(block_ack_msg).await;

    Ok(())
}
