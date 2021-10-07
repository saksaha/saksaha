use super::{
    credential::Credential,
    listener::Listener,
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
    pconfig::PersistedP2PConfig,
};
use logger::log;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub enum HostStatus<E> {
    Launched,

    SetupFailed(E),
}

pub struct Host {
    task_mng: Arc<TaskManager>,
    credential: Arc<Credential>,
}

impl Host {
    pub fn new(
        task_mng: Arc<TaskManager>,
        p2p_config: PersistedP2PConfig,
    ) -> Result<Host> {
        let secret = p2p_config.secret.to_owned();
        let public_key = p2p_config.public_key.to_owned();

        let credential = match Credential::new(secret, public_key) {
            Ok(c) => Arc::new(c),
            Err(err) => return Err(err),
        };

        let host = Host {
            task_mng,
            credential,
        };

        Ok(host)
    }

    pub async fn start(
        &self,
        rpc_port: u16,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
    ) -> HostStatus<Error> {
        let disc_listener = match Listener::new_tcp(disc_port).await {
            Ok(l) => l,
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let peer_op_listener = match Listener::new_tcp(None).await {
            Ok(l) => l,
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let credential = self.credential.clone();
        let peer_store = PeerStore::new(10, bootstrap_urls);
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
            peer_op_listener,
        );

        let disc = Disc::new(
            disc_port,
            peer_store.clone(),
            self.task_mng.clone(),
            credential.clone(),
            Arc::new(Mutex::new(disc_wakeup_rx)),
            Arc::new(peer_op_wakeup_tx),
            disc_listener,
        );

        let sync = Sync::new();

        let handshake_started = tokio::spawn(async move {
            let port = match handshake.start().await {
                handshake::Status::Launched(port) => port,
                handshake::Status::SetupFailed(err) => return Err(err),
            };

            Ok(port)
        });

        let peer_op_port = match handshake_started.await {
            Ok(p) => match p {
                Ok(p) => p,
                Err(err) => return HostStatus::SetupFailed(err),
            },
            Err(err) => return HostStatus::SetupFailed(err.into()),
        };

        tokio::spawn(async move {
            disc.start(peer_op_port).await;
        });

        HostStatus::Launched
    }

}
