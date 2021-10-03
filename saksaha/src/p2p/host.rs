use super::{
    credential::Credential, discovery::Disc, peer::peer_store::PeerStore,
    peer_op::PeerOp,
};
use crate::{
    common::Result,
    err_res, msg_err,
    node::task_manager::{MsgKind, TaskManager},
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

pub struct Host {
    rpc_port: u16,
    disc_port: u16,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
    public_key: String,
}

impl Host {
    pub fn new(
        rpc_port: u16,
        disc_port: u16,
        bootstrap_peers: Option<Vec<String>>,
        task_mng: Arc<TaskManager>,
        secret: String,
        public_key: String,
    ) -> Result<Host> {
        let host = Host {
            rpc_port,
            disc_port,
            bootstrap_peers,
            task_mng,
            secret,
            public_key,
        };

        Ok(host)
    }
}

impl Host {
    pub async fn start(&self) {
        log!(DEBUG, "Start host...\n");

        let credential = match Credential::new(
            self.secret.to_owned(),
            self.public_key.to_owned(),
        ) {
            Ok(sk) => sk,
            Err(err) => {
                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error creating secret key, err: {}",
                    err
                );

                return self.task_mng.send(msg).await;
            }
        };

        let peer_store = Arc::new(PeerStore::new(10));
        let (dial_loop_tx, dial_loop_rx) = mpsc::channel::<usize>(5);
        let rpc_port = self.rpc_port;
        let task_mng = self.task_mng.clone();

        let peer_op = PeerOp::new(
            peer_store.clone(),
            Arc::new(dial_loop_tx),
            rpc_port,
            task_mng,
        );

        let peer_op_port = tokio::spawn(async move {
            let port = peer_op.start().await;

            let port = match port {
                Ok(p) => p,
                Err(err) => {
                    return err_res!("Error starting peer op, err: {}", err);
                }
            };

            Ok(port)
        });

        let peer_op_port = match peer_op_port.await {
            Ok(p) => p,
            Err(err) => {
                log!(DEBUG, "Error joining peer op start thread, err: {}", err);
                return;
            }
        };

        println!("22, {}", peer_op_port.unwrap());

        // let peer_op_port = match peer_op_port_rx.await {
        //     Ok(port) => port,
        //     Err(err) => {
        //         let msg = msg_err!(
        //             MsgKind::SetupFailure,
        //             "Error retrieving peer op port, err: {}",
        //             err,
        //         );

        //         return self.task_mng.send(msg).await;
        //     }
        // };

        // let disc = Disc::new(
        //     self.disc_port,
        //     peer_op_port,
        //     self.bootstrap_peers.to_owned(),
        //     peer_store.clone(),
        //     self.task_mng.clone(),
        //     Arc::new(credential),
        //     Arc::new(Mutex::new(dial_loop_rx)),
        // );

        // tokio::spawn(async move {
        //     disc.start().await;
        // });
    }
}
