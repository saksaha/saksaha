use super::{
    credential::Credential, discovery::Disc, peer::peer_store::PeerStore,
    peer_op::PeerOp,
};
use crate::{
    common::SakResult,
    crypto::Crypto,
    err_res, msg_err,
    node::task_manager::{Msg, MsgKind, TaskManager},
};
use logger::log;
use std::{sync::Arc, time::Duration};
use tokio::{
    sync::{mpsc, oneshot, Mutex},
    task::JoinHandle,
};

pub struct Host {
    rpc_port: u16,
    disc_port: u16,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
}

impl Host {
    pub fn new(
        rpc_port: u16,
        disc_port: u16,
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

        let (peer_op_port_tx, mut peer_op_port_rx) = mpsc::channel(1);
        let (dial_loop_tx, dial_loop_rx) = mpsc::channel::<usize>(5);

        let peer_op = PeerOp::new(
            peer_store_clone,
            Arc::new(peer_op_port_tx),
            Arc::new(dial_loop_tx),
        );

        tokio::spawn(async move {
            peer_op.start().await;
        });

        let peer_op_port = match peer_op_port_rx.recv().await {
            Some(port) => port,
            None => {
                log!(DEBUG, "Fatal error. Cannot retrieve peer op port\n",);

                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error retrieving peer op port",
                );

                self.task_mng
                    .send(msg)
                    .await
                    .expect("Fatal message should be delivered");

                return;
            }
        };

        let peer_store_clone = peer_store.clone();
        let task_mng = self.task_mng.clone();

        let disc = Disc::new(
            self.disc_port,
            peer_op_port,
            self.bootstrap_peers.to_owned(),
            peer_store_clone,
            task_mng,
            Arc::new(credential),
            Arc::new(Mutex::new(dial_loop_rx)),
        );

        tokio::spawn(async move {
            disc.start().await;
        });
    }
}
