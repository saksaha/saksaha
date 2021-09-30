use super::{discovery::Disc, peer_op::PeerOp, peer_store::PeerStore};
use crate::{
    common::SakResult,
    err_res,
    node::task_manager::{Msg, MsgKind, TaskManager},
    sync::Sync,
};
use logger::log;
use std::sync::Arc;
use tokio::{
    sync::{mpsc::Sender, Mutex},
    task::JoinHandle,
};

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
}

impl Host {
    pub fn new(
        rpc_port: usize,
        disc_port: usize,
        bootstrap_peers: Option<Vec<String>>,
        public_key: String,
        secret: String,
        task_mng: Arc<TaskManager>,
    ) -> SakResult<Host> {
        let host = Host {
            rpc_port,
            disc_port,
            bootstrap_peers,
            task_mng,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) {
        log!(DEBUG, "Start host...\n");

        let peer_store = Arc::new(PeerStore::new(10));
        let peer_store_clone = peer_store.clone();

        let task_mng = self.task_mng.clone();

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
            task_mng,
        );

        tokio::spawn(async move {
            disc.start().await;
        });

        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(peer_store_clone);

        tokio::spawn(async move {
            peer_op.start().await;
        });
    }
}
