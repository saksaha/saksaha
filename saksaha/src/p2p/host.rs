pub use super::status::Status;
use super::{
    credential::Credential, discovery::Disc, peer::peer_store::PeerStore,
    peer_op::PeerOp,
};
use crate::{
    common::{Error, Result},
    err, msg_err,
    node::task_manager::{MsgKind, TaskManager},
    p2p::peer_op,
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};

pub struct Components {
    peer_op: PeerOp,
    disc: Disc,
}

pub struct Host {
    disc_port: Option<u16>,
    bootstrap_peers: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
    public_key: String,
}

impl Host {
    pub fn new(
        disc_port: Option<u16>,
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

    pub async fn start_components(
        &self,
        components: Arc<Components>,
    ) -> Result<()> {
        let c = components.clone();
        let peer_op_port = tokio::spawn(async move {
            let port = match c.peer_op.start().await {
                peer_op::Status::Launched(port) => port,
                peer_op::Status::SetupFailed(err) => return Err(err),
            };

            Ok(port)
        });

        let peer_op_port = match peer_op_port.await {
            Ok(p) => match p {
                Ok(p) => p,
                Err(err) => {
                    return Err(err);
                }
            },
            Err(err) => {
                return Err(err.into());
            }
        };

        let c = components.clone();
        tokio::spawn(async move {
            c.disc.start(peer_op_port).await;
        });

        Ok(())
    }

    pub fn make_components(&self, rpc_port: u16) -> Result<Components> {
        let credential = match Credential::new(
            self.secret.to_owned(),
            self.public_key.to_owned(),
        ) {
            Ok(sk) => sk,
            Err(err) => {
                return Err(err);
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

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store.clone(),
            self.task_mng.clone(),
            Arc::new(credential),
            Arc::new(Mutex::new(dial_loop_rx)),
        );

        let components = Components { peer_op, disc };

        Ok(components)
    }

    pub async fn start(&self, rpc_port: u16) -> Status<Error> {
        log!(DEBUG, "Start host...\n");

        let components = match self.make_components(rpc_port) {
            Ok(c) => Arc::new(c),
            Err(err) => return Status::SetupFailed(err),
        };

        match self.start_components(components).await {
            Ok(_) => (),
            Err(err) => return Status::SetupFailed(err),
        };

        Status::Launched
    }
}
