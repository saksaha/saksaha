use crate::{
    machine::Machine,
    node::{task::NodeTask, SaksahaNodeError},
};
use futures::{stream::SplitSink, SinkExt};
use log::{info, warn};
use sak_p2p_transport::{
    BlockHashSynMsg, Msg, TxHashSyncMsg, UpgradedConn, UpgradedP2PCodec,
};
use sak_task_queue::TaskQueue;
use sak_types::{BlockHash, BlockHeight, TxHash};
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLockWriteGuard};
use tokio_util::codec::Framed;

pub(super) async fn handle_tx_pool_stat<'a>(
    _machine: &Machine,
    new_tx_hashes: Vec<TxHash>,
    node_task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<(), SaksahaNodeError> {
    // node_task_queue
    //     .push_back(NodeTask::SendTxHashSyn {
    //         tx_hashes: new_tx_hashes,
    //     })
    //     .await

    // match conn
    //     .send(Msg::TxHashSyn(TxHashSynMsg {
    //         tx_hashes: new_tx_hashes,
    //     }))
    //     .await
    // {
    //     Ok(_) => {
    //         info!("Sending TxHashSyn, dst public_key: {}", public_key);
    //     }
    //     Err(err) => {
    //         warn!(
    //             "Failed to request to synchronize with peer node, err: {}",
    //             err,
    //         );
    //     }
    // };
    Ok(())
}

pub(super) async fn handle_new_blocks_ev<'a>(
    _machine: &Machine,
    new_blocks: Vec<(BlockHeight, BlockHash)>,
    node_task_queue: &Arc<TaskQueue<NodeTask>>,
) -> Result<(), SaksahaNodeError> {
    node_task_queue
        .push_back(NodeTask::SendBlockHashSyn {
            new_blocks: new_blocks.clone(),
        })
        .await?;

    // match conn
    //     // .socket
    //     .send(Msg::BlockHashSyn(BlockHashSynMsg {
    //         new_blocks: new_blocks.clone(),
    //     }))
    //     .await
    // {
    //     Ok(_) => {
    //         info!("Sending block hash syn, dst public_key: {}", public_key);
    //     }
    //     Err(err) => {
    //         warn!(
    //             "Failed to request to synchronize with peer node, err: {}",
    //             err,
    //         );
    //     }
    // };
    Ok(())
}

// pub(super) async fn handle_new_peers_ev<'a>(
//     // public_key: &str,
//     conn: &'a mut RwLockWriteGuard<'_, UpgradedConn>,
//     machine: &Machine,
// ) {
//     let blocks = machine
//         .blockchain
//         .dist_ledger
//         .apis
//         .get_entire_block_info_list()
//         .await
//         .unwrap_or(vec![]);

//     match conn
//         // .socket
//         .send(Msg::BlockHashSyn(BlockHashSynMsg { new_blocks: blocks }))
//         .await
//     {
//         Ok(_) => {
//             // info!("Sending block hash syn, dst public_key: {}", public_key);
//         }
//         Err(err) => {
//             warn!(
//                 "Failed to request to synchronize with peer node, err: {}",
//                 err,
//             );
//         }
//     };
// }
