use super::NodeTask;
use crate::{machine::Machine, node::msg_handle};
use async_trait::async_trait;
use log::{debug, error, warn};
use sak_p2p_peertable::{Peer, PeerStatus, PeerTable};
use sak_p2p_transport::{
    handshake::{self, HandshakeInitArgs},
    Conn, Msg, TxHashSyncMsg, TxSynMsg, UpgradedConn,
};
use sak_task_queue::{TaskQueue, TaskQueueError};
use sak_types::TxCandidate;
use std::sync::Arc;
use tokio::{
    net::TcpStream,
    sync::{RwLock, RwLockWriteGuard},
};

pub(in crate::node) async fn handle_task<'a>(
    task: NodeTask,
    task_queue: &Arc<TaskQueue<NodeTask>>,
    conn_lock: RwLockWriteGuard<'a, UpgradedConn>,
    machine: &Arc<Machine>,
) {
    println!("handle new task: {}", task);

    let task_type = task.to_string();

    let res = match task {
        NodeTask::SendTxHashSyn { tx_hashes } => {
            msg_handle::send_tx_hash_syn(conn_lock, tx_hashes, task_queue).await
        }
        NodeTask::SendTxSyn { tx_hashes } => {
            msg_handle::send_tx_syn(conn_lock, tx_hashes, &machine).await
        }
        NodeTask::SendBlockHashSyn { new_blocks } => {
            // Ok(())
            Ok(())
        }
    };

    if let Err(err) = res {
        warn!("Task handle failed, task: {}, err: {}", task_type, err);
    }
}

// pub(in crate::node) struct NodeTaskHandler {
//     // pub peer_table: Arc<PeerTable>,
//     pub peer: Arc<Peer>,
//     pub machine: Arc<Machine>,
// }

// #[async_trait]
// impl TaskHandler<NodeTask> for NodeTaskHandler {
//     async fn handle_task(
//         &self,
//         task: NodeTask,
//         task_queue: &Arc<TaskQueue<NodeTask>>,
//     ) {
//         println!("handle new task: {}", task);

//         let res = match task {
//             NodeTask::SendTxHashSyn { tx_hashes } => {
//                 msg_handle::send_tx_hash_syn(&self.peer, tx_hashes, task_queue)
//                     .await
//             }
//             NodeTask::SendTxSyn { tx_hashes } => {
//                 msg_handle::send_tx_syn(&self.peer, tx_hashes, &self.machine)
//                     .await
//                 // handle_send_tx_syn(
//                 //     tx_candidates,
//                 //     her_public_key,
//                 //     &self.peer_table,
//                 // )
//                 // .await
//             }
//             NodeTask::SendBlockHashSyn { new_blocks } => Ok(()),
//         };

//         if let Err(err) = res {
//             warn!("Task handle failed, err: {}", err);
//         }
//     }
// }

// async fn handle_send_tx_syn(
//     tx_candidates: Vec<TxCandidate>,
//     her_public_key: Option<String>,
//     peer_table: &Arc<PeerTable>,
// ) -> Result<(), TaskQueueError> {
//     if let Some(ref her_pk) = her_public_key {
//         let peer = peer_table.get_mapped_peer(&her_pk).await.ok_or(format!(
//             "peer does not exist, key: {:?}",
//             &her_public_key
//         ))?;

//         msg_handle::send_tx_syn(&peer, tx_candidates).await?;
//     } else {
//         let peer_map_lock = peer_table.get_peer_map().read().await;

//         for (_pk, peer) in peer_map_lock.iter() {
//             msg_handle::send_tx_syn(peer, tx_candidates.clone()).await?;
//         }
//     }

//     Ok(())
// }
