pub use super::status::Status;
use super::{
    credential::Credential,
    ops::{
        discovery::Disc,
        handshake::{self, Handshake},
        sync::Sync,
    },
    peer::peer_store::PeerStore,
};
use crate::{
    common::{Error, Result},
    err,
    node::task_manager::TaskManager,
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

struct Components {
    handshake: Handshake,
    sync: Sync,
    disc: Disc,
}

pub struct Host {
    disc_port: Option<u16>,
    bootstrap_urls: Option<Vec<String>>,
    task_mng: Arc<TaskManager>,
    secret: String,
    public_key: String,
}

impl Host {
    pub fn new(
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        task_mng: Arc<TaskManager>,
        secret: String,
        public_key: String,
    ) -> Host {
        let host = Host {
            disc_port,
            bootstrap_urls,
            task_mng,
            secret,
            public_key,
        };

        host
    }

    async fn start_components(&self, components: Components) -> Result<()> {
        let handshake = components.handshake;
        let peer_op_port = tokio::spawn(async move {
            let port = match handshake.start().await {
                handshake::Status::Launched(port) => port,
                handshake::Status::SetupFailed(err) => return Err(err),
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
            Ok(c) => Arc::new(c),
            Err(err) => {
                return Err(err);
            }
        };

        let peer_store = PeerStore::new(10, self.bootstrap_urls.clone());
        let peer_store = Arc::new(peer_store);
        let (disc_wakeup_tx, disc_wakeup_rx) = mpsc::channel::<usize>(5);
        let (peer_op_wakeup_tx, peer_op_wakeup_rx) = mpsc::channel::<usize>(5);
        let task_mng = self.task_mng.clone();

        let handshake = Handshake::new(
            peer_store.clone(),
            Arc::new(disc_wakeup_tx),
            rpc_port,
            task_mng,
            Arc::new(Mutex::new(peer_op_wakeup_rx)),
            credential.clone(),
        );

        let disc = Disc::new(
            self.disc_port,
            peer_store.clone(),
            self.task_mng.clone(),
            credential.clone(),
            Arc::new(Mutex::new(disc_wakeup_rx)),
            Arc::new(peer_op_wakeup_tx),
        );

        let sync = Sync::new();

        let components = Components {
            handshake,
            disc,
            sync,
        };

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
