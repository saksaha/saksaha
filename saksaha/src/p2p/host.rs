use super::{
    credential::Credential,
    listener::Listener,
    ops::{
        discovery::{self, Disc},
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
}

impl Host {
    pub fn new(task_mng: Arc<TaskManager>) -> Result<Host> {
        let host = Host { task_mng };

        Ok(host)
    }

    fn make_credential(p2p_config: PersistedP2PConfig) -> Result<Credential> {
        let secret = p2p_config.secret.to_owned();
        let public_key = p2p_config.public_key.to_owned();

        let credential = match Credential::new(secret, public_key) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        Ok(credential)
    }

    fn make_peer_store(
        bootstrap_urls: Option<Vec<String>>,
    ) -> Result<PeerStore> {
        let peer_store = match PeerStore::new(10, bootstrap_urls) {
            Ok(p) => p,
            Err(err) => return Err(err)
        };

        Ok(peer_store)
    }

    pub async fn start(
        &self,
        p2p_config: PersistedP2PConfig,
        rpc_port: u16,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
    ) -> HostStatus<Error> {
        let credential = match Host::make_credential(p2p_config) {
            Ok(c) => Arc::new(c),
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let (disc_listener, disc_port) =
        match Listener::new_tcp(disc_port).await {
            Ok(l) => l,
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let (peer_op_listener, peer_op_port) =
        match Listener::new_tcp(None).await {
            Ok(l) => l,
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let credential_clone = credential.clone();
        let peer_store = match Host::make_peer_store(bootstrap_urls) {
            Ok(p) => Arc::new(p),
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let (disc_wakeup_tx, disc_wakeup_rx) = mpsc::channel::<usize>(5);
        let (peer_op_wakeup_tx, peer_op_wakeup_rx) = mpsc::channel::<usize>(5);

        let handshake = Handshake::new(self.task_mng.clone());
        let handshake_started = handshake.start(
            peer_store.clone(),
            Arc::new(disc_wakeup_tx),
            rpc_port,
            Arc::new(Mutex::new(peer_op_wakeup_rx)),
            credential_clone,
            peer_op_listener,
        );

        match handshake_started.await {
            handshake::Status::Launched => (),
            handshake::Status::SetupFailed(err) => {
                return HostStatus::SetupFailed(err);
            }
        };

        let credential_clone = credential.clone();
        let disc = Disc::new(self.task_mng.clone());
        let disc_started = disc.start(
            peer_op_port,
            peer_store.clone(),
            credential_clone,
            Arc::new(Mutex::new(disc_wakeup_rx)),
            Arc::new(peer_op_wakeup_tx),
            disc_listener,
            disc_port,
        );

        match disc_started.await {
            discovery::Status::Launched => (),
            discovery::Status::SetupFailed(err) => {
                return HostStatus::SetupFailed(err);
            }
        };

        let sync = Sync::new();

        HostStatus::Launched
    }
}
