use crate::node::SaksahaNodeError;

use super::NodeTask;
use async_trait::async_trait;
use log::{debug, error, warn};
use sak_p2p_peertable::{Peer, PeerStatus, PeerTable};
use sak_p2p_transport::{
    handshake::{self, HandshakeInitArgs},
    Conn,
};
use sak_task_queue::TaskHandler;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock};

pub(in crate::node) struct NodeTaskHandler {
    pub peer_table: Arc<PeerTable>,
}

#[async_trait]
impl TaskHandler<NodeTask> for NodeTaskHandler {
    async fn handle_task(
        &self,
        task: NodeTask,
    ) -> Result<(), SaksahaNodeError> {
        println!("handle new task: {}", task);

        match task {
            NodeTask::SendHello { her_public_key } => {
                self.peer_table.get_mapped_peer(&her_public_key)
            }
            NodeTask::SendTxSyn {
                tx_candidates,
                her_public_key,
            } => {}
            NodeTask::SendTxHashSyn {
                tx_hashes,
                her_public_key,
            } => {}
            NodeTask::SendBlockHashSyn {
                new_blocks,
                her_public_key,
            } => {}
        };

        Ok(())
    }
}
