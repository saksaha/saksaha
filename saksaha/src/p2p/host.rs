use super::{
    credential::Credential, discovery::Disc, peer_op::PeerOp,
    peer_store::PeerStore,
};
use crate::{
    common::SakResult,
    crypto::Crypto,
    err_res, msg_err,
    node::task_manager::{Msg, MsgKind, TaskManager},
};
use logger::log;
use std::sync::Arc;
use tokio::{sync::Mutex, task::JoinHandle};

pub struct Host {
    rpc_port: usize,
    disc_port: usize,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
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
            secret,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) {
        log!(DEBUG, "Start host...\n");

        let credential = match Credential::new(self.secret.to_owned()) {
            Ok(sk) => sk,
            Err(err) => {
                log!(
                    DEBUG,
                    "Fatal error. Cannot create secret key, err: {}",
                    err
                );

                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error creating secret key, err: {}",
                    err
                );

                self.task_mng
                    .send(msg)
                    .await
                    .expect("Fatal message should be delivered");

                return;
            }
        };

        let peer_store = Arc::new(PeerStore::new(10));
        let peer_store_clone = peer_store.clone();
        let peer_op = PeerOp::new(peer_store_clone);

        tokio::spawn(async move {
            peer_op.start().await;
        });

        let peer_store_clone = peer_store.clone();

        let task_mng = self.task_mng.clone();

        let disc = Disc::new(
            self.disc_port,
            peer_op_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
            task_mng,
            Arc::new(credential),
        );

        tokio::spawn(async move {
            disc.start().await;
        });
    }
}
