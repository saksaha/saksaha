pub use super::status::Status;
use super::{
    credential::Credential, discovery::Disc, peer::peer_store::PeerStore,
    peer_op::PeerOp,
};
use crate::{
    common::{Error, Result},
    err, msg_err,
    node::task_manager::{MsgKind, TaskManager},
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

pub struct Components {
    peer_op: PeerOp,
    disc: Disc,
}

pub struct Host {
    disc_port: u16,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
    public_key: String,
}

impl Host {
    pub fn new(
        disc_port: u16,
        bootstrap_peers: Option<Vec<String>>,
        task_mng: Arc<TaskManager>,
        secret: String,
        public_key: String,
    ) -> Host {
        let host = Host {
            disc_port,
            bootstrap_peers,
            task_mng,
            secret,
            public_key,
        };

        host
    }

    pub fn make_components(&self) -> Result<Components> {
        let credential = match Credential::new(
            self.secret.to_owned(),
            self.public_key.to_owned(),
        ) {
            Ok(sk) => sk,
            Err(err) => {
                return Status::SetupFailed(err);
            }
        };

        let peer_store = Arc::new(PeerStore::new(10));
        let (dial_loop_tx, dial_loop_rx) = mpsc::channel::<usize>(5);
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
                    return err!("Error starting peer op, err: {}", err);
                }
            };

            Ok(port)
        });

        let peer_op_port = match peer_op_port.await {
            Ok(p) => p,
            Err(err) => {
                log!(DEBUG, "Error joining peer op start thread, err: {}", err);
                return Status::SetupFailed(err.into());
            }
        };

        println!("22, {}", peer_op_port.unwrap());

        Status::Launched

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

        let components = Components {

        }

        Ok(components)
    }

    pub async fn start(&self, rpc_port: u16) -> Status<Error> {
        log!(DEBUG, "Start host...\n");

        let components = self.make_components();

    }
}
