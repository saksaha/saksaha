use super::NodeTask;
use async_trait::async_trait;
use log::{debug, error, warn};
use sak_p2p_peertable::{Peer, PeerStatus};
use sak_p2p_transport::{
    handshake::{self, HandshakeInitArgs},
    Conn,
};
use sak_task_queue::TaskHandler;
use std::sync::Arc;
use tokio::{net::TcpStream, sync::RwLock};

pub(in crate::node) struct NodeTaskHandler {}

#[async_trait]
impl TaskHandler for NodeTaskHandler {
    async fn handle_task(&self) {}
}
