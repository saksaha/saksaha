use super::{
    credential::Credential,
    listener::{self, Listener},
};
use crate::{
    p2p::listener::error::ListenerError, pconfig::PersistedP2PConfig,
    peer::peer_store::PeerStore,
};
use log::error;
use saksaha_discovery::{Disc, identity::Identity};
use std::sync::Arc;

pub struct Host {}

impl Host {
    pub fn new() -> Host {
        let host = Host {};
        host
    }

    fn make_credential(
        p2p_config: PersistedP2PConfig,
    ) -> Result<Box<dyn Identity + Send + Sync>, String> {
        let secret = p2p_config.secret.to_owned();
        let public_key = p2p_config.public_key.to_owned();

        let credential = match Credential::new(secret, public_key) {
            Ok(c) => c,
            Err(err) => return Err(err),
        };

        Ok(Box::new(credential))
    }

    fn make_peer_store() -> Result<PeerStore, String> {
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
        default_bootstrap_urls: &str,
    ) -> Result<(), String> {
        let credential = match Host::make_credential(p2p_config) {
            Ok(c) => Arc::new(c),
            Err(err) => return Err(err),
        };

        let peer_store = match Host::make_peer_store() {
            Ok(p) => Arc::new(p),
            Err(err) => return Err(err),
        };

        let p2p_listener = Listener::new();
        let p2p_listener_port = match p2p_listener
            .start(None, peer_store.clone(), rpc_port, credential.clone())
            .await
        {
            Ok(port) => port,
            Err(err) => match err {
                ListenerError::SetupFail(err) => {
                    error!("Couldn't start listener, err: {}", err);

                    return Err(err);
                }
            },
        };

        let disc = match Disc::init(
            credential.clone(),
            disc_port,
            p2p_listener_port,
            bootstrap_urls,
            default_bootstrap_urls,
        )
        .await
        {
            Ok(d) => d,
            Err(err) => {
                return Err(format!("Can't start discovery, err: {}", err))
            }
        };

        let table = match disc.start().await {
            Ok(table) => table,
            Err(err) => {
                return Err(err);
            }
        };

        // let dialer = Dialer::new(table);
        // dialer.schedule();

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

        Ok(())
    }
}
