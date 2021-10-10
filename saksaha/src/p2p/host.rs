use super::{
    credential::Credential,
    discovery::Disc,
    listener::{self, Listener},
    ops::{
        handshake::{self, Handshake},
    },
    peer::peer_store::PeerStore,
};
use crate::{common::{Error, Result}, err, node::task_manager::TaskManager, p2p::discovery, pconfig::PersistedP2PConfig};
use futures::stream::{FuturesOrdered, FuturesUnordered};
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
        // bootstrap_urls: Option<Vec<String>>,
    ) -> Result<PeerStore> {
        let peer_store = match PeerStore::new(10) {
            Ok(p) => p,
            Err(err) => return Err(err),
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

        let peer_store = match Host::make_peer_store() {
            Ok(p) => Arc::new(p),
            Err(err) => return HostStatus::SetupFailed(err),
        };

        let p2p_listener = Listener::new();
        let p2p_listener_port = match p2p_listener
            .start(
                None,
                peer_store.clone(),
                rpc_port,
                credential.clone(),
            )
            .await
        {
            listener::Status::Launched(port) => port,
            listener::Status::SetupFailed(err) => {
                log!(DEBUG, "Couldn't start listener, err: {}\n", err);

                return HostStatus::SetupFailed(err)
            }
        };

        let disc = Disc::new();
        match disc.start(
            disc_port,
            p2p_listener_port,
            peer_store.clone(),
            credential.clone(),
            bootstrap_urls,
        ).await {
            discovery::Status::Launched => (),
            discovery::Status::SetupFailed(err) => {
                return HostStatus::SetupFailed(err);
            }
        };

        // let credential_clone = credential.clone();

        // let handshake = Handshake::new(self.task_mng.clone());
        // let handshake_started = handshake.start(
        //     peer_store.clone(),
        //     Arc::new(disc_wakeup_tx),
        //     rpc_port,
        //     Arc::new(Mutex::new(peer_op_wakeup_rx)),
        //     credential_clone,
        //     peer_op_listener,
        // );

        // match handshake_started.await {
        //     handshake::Status::Launched => (),
        //     handshake::Status::SetupFailed(err) => {
        //         return HostStatus::SetupFailed(err);
        //     }
        // };

        HostStatus::Launched
    }
}
