pub use super::status::Status;
use super::{
    credential::Credential, discovery::Disc, peer::peer_store::PeerStore,
    peer_op::PeerOp,
};
use crate::{
    common::{Error, Result},
    err,
    node::task_manager::TaskManager,
    p2p::peer_op,
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

struct Components {
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

    async fn start_components(&self, components: Components) -> Result<()> {
        let peer_op = components.peer_op;
        let peer_op_port = tokio::spawn(async move {
            let port = match peer_op.start().await {
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

        let disc = components.disc;
        tokio::spawn(async move {
            disc.start(peer_op_port).await;
        });

        Ok(())
    }

    fn make_components(&self, rpc_port: u16) -> Result<Components> {
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
        let (disc_wakeup_tx, disc_wakeup_rx) = mpsc::channel::<usize>(5);
        let (peer_op_wakeup_tx, peer_op_wakeup_rx) = mpsc::channel::<usize>(5);
        let task_mng = self.task_mng.clone();

        let peer_op = PeerOp::new(
            peer_store.clone(),
            Arc::new(disc_wakeup_tx),
            rpc_port,
            task_mng,
            Arc::new(Mutex::new(peer_op_wakeup_rx)),
        );

        let disc = Disc::new(
            self.disc_port,
            self.bootstrap_peers.to_owned(),
            peer_store.clone(),
            self.task_mng.clone(),
            Arc::new(credential),
            Arc::new(Mutex::new(disc_wakeup_rx)),
            Arc::new(peer_op_wakeup_tx),
        );

        let components = Components { peer_op, disc };

        Ok(components)
    }

    pub async fn start(&self, rpc_port: u16) -> Status<Error> {
        log!(DEBUG, "Start host...\n");

        let components = match self.make_components(rpc_port) {
            Ok(c) => c,
            Err(err) => return Status::SetupFailed(err),
        };

        match self.start_components(components).await {
            Ok(_) => (),
            Err(err) => return Status::SetupFailed(err),
        };

        Status::Launched
    }
}
