use crate::{machine::Machine, SaksahaError};
use futures::{stream::SplitSink, SinkExt};
use log::{debug, info, warn};
use sak_p2p_transport::{
    BlockHashSynMsg, BlockSynMsg, Msg, TxHashSynMsg, TxSynMsg, UpgradedConn,
    UpgradedP2PCodec,
};
use tokio::{net::TcpStream, sync::RwLockWriteGuard};
use tokio_util::codec::Framed;

pub(crate) async fn handle_msg<'a>(
    msg: Msg,
    public_key: &str,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaError> {
    match msg {
        Msg::TxHashSyn(tx_hash_syn) => {
            handle_tx_hash_syn(public_key, tx_hash_syn, machine, conn).await
        }
        Msg::TxHashAck(tx_hash_ack) => {
            handle_tx_hash_ack(public_key, tx_hash_ack, machine, conn).await;
        }
        Msg::TxSyn(tx_syn) => {
            handle_tx_syn(tx_syn, machine).await?;
        }
        Msg::BlockHashSyn(block_hash_syn_msg) => {
            handle_block_hash_syn(block_hash_syn_msg, machine, conn).await?;
        }
        Msg::BlockSyn(block_syn_msg) => {
            handle_block_syn(block_syn_msg, machine).await?;
        }
        Msg::BlockHashAck(block_hash_syn_msg) => {
            let _ =
                handle_block_hash_ack(block_hash_syn_msg, machine, conn).await;
        }
        _ => {
            warn!("Msg not valid at this stage, discarding, msg: {:?}", msg);
        }
    };

    Ok(())
}

async fn handle_tx_hash_ack<'a>(
    public_key: &str,
    tx_hash_ack: TxHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
) {
    let tx_candidates = machine
        .blockchain
        .dist_ledger
        .apis
        .get_txs_from_pool(tx_hash_ack.tx_hashes)
        .await;

    if !tx_candidates.is_empty() {
        match conn
            // .socket
            .send(Msg::TxSyn(TxSynMsg { tx_candidates }))
            .await
        {
            Ok(_) => {
                info!("Sending TxSyn, public_key: {}", public_key);
            }
            Err(err) => {
                info!("Failed to handle TxHashAck, err: {}", err,);
            }
        }
    }
}

async fn handle_tx_syn(
    tx_syn: TxSynMsg,
    machine: &Machine,
) -> Result<(), SaksahaError> {
    machine
        .blockchain
        .dist_ledger
        .apis
        .insert_into_pool(tx_syn.tx_candidates)
        .await;

    Ok(())
}

async fn handle_tx_hash_syn<'a>(
    public_key: &str,
    tx_hash_syn_msg: TxHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
) {
    let txs_to_request = machine
        .blockchain
        .dist_ledger
        .apis
        .get_tx_pool_diff(tx_hash_syn_msg.tx_hashes)
        .await;

    match conn
        // .socket
        .send(Msg::TxHashAck(TxHashSynMsg {
            tx_hashes: txs_to_request,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to handle TxHashSyn msg, err: {}", err,);
        }
    };
}

async fn handle_block_hash_syn<'a>(
    block_hash_syn_msg: BlockHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let (_, latest_block_hash) = machine
        .blockchain
        .dist_ledger
        .apis
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
        // .socket
        .send(Msg::BlockHashAck(BlockHashSynMsg {
            new_blocks: blocks_to_req,
        }))
        .await
    {
        Ok(_) => {}
        Err(err) => {
            warn!("Failed to handle BlockHashSyn, err: {}", err,);
        }
    };

    Ok(())
}

async fn handle_block_syn(
    block_syn_msg: BlockSynMsg,
    machine: &Machine,
) -> Result<(), SaksahaError> {
    let blocks = block_syn_msg.blocks;

    let latest_block_height = machine
        .blockchain
        .dist_ledger
        .apis
        .get_latest_block_height()?
        .unwrap_or(0);

    for (block, txs) in blocks {
        if block.block_height != (latest_block_height + 1) {
            return Err("received not continuous block height".into());
        }

        machine
            .blockchain
            .dist_ledger
            .apis
            .sync_block(block, txs)
            .await?;
    }

    Ok(())
}

async fn handle_block_hash_ack<'a>(
    block_hash_syn_msg: BlockHashSynMsg,
    machine: &Machine,
    conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
) -> Result<(), SaksahaError> {
    let new_blocks = block_hash_syn_msg.new_blocks;

    let block_hashes: Vec<&String> = new_blocks
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

    if !blocks_to_send.is_empty() {
        match conn
            // .socket
            .send(Msg::BlockSyn(BlockSynMsg {
                blocks: blocks_to_send,
            }))
            .await
        {
            Ok(_) => {}
            Err(err) => {
                info!("Failed to handle blockHashAck, err: {}", err,);
            }
        }
    }

    Ok(())
}
