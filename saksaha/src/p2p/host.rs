use super::{
    credential::Credential,
    listener::{self, Listener},
};
use crate::{pconfig::PersistedP2PConfig, peer::peer_store::PeerStore};
use log::error;
use saksaha_p2p_discovery::Disc;
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::net::TcpListener;

pub struct Host {
    disc: Arc<Disc>,
}

impl Host {
    pub async fn init(
        p2p_config: PersistedP2PConfig,
        rpc_port: u16,
        p2p_port: Option<u16>,
        disc_port: Option<u16>,
        bootstrap_urls: Option<Vec<String>>,
        default_bootstrap_urls: &str,
    ) -> Result<Host, String> {
        let p2p_port = match p2p_port {
            Some(p) => p,
            None => 0,
        };

        let local_addr = format!("127.0.0.1:{}", p2p_port);

        let (tcp_listener, local_addr) = match TcpListener::bind(local_addr)
            .await
        {
            Ok(listener) => match listener.local_addr() {
                Ok(local_addr) => {
                    // log!(DEBUG, "Listener created, addr: {}", local_addr);

                    (listener, local_addr)
                }
                Err(err) => {
                    return Err(format!(
                        "Can't get local address of p2p listener, err: {}",
                        err
                    ))
                }
            },
            Err(err) => {
                return Err(format!("Can't bind tcp listener, err: {}", err))
            }
        };

        let p2p_listener = Listener::new(tcp_listener);
        let credential = Host::make_credential(p2p_config)?;
        let peer_store = Host::make_peer_store()?;

        let p2p_listener_port = p2p_listener
            .start(None, peer_store.clone(), rpc_port, credential.clone())
            .await?;

        let disc = Disc::init(
            credential.clone(),
            disc_port,
            p2p_listener_port,
            bootstrap_urls,
            default_bootstrap_urls,
        )
        .await?;

        let host = Host {
            disc: Arc::new(disc),
        };

        Ok(host)
    }

    fn make_credential(
        p2p_config: PersistedP2PConfig,
    ) -> Result<Arc<Box<dyn Identity + Send + Sync>>, String> {
        let secret = p2p_config.secret.to_owned();
        let public_key = p2p_config.public_key.to_owned();

        let credential = match Credential::new(secret, public_key) {
            Ok(c) => Box::new(c),
            Err(err) => return Err(err),
        };

        Ok(Arc::new(credential))
    }

    fn make_peer_store() -> Result<Arc<PeerStore>, String> {
        let peer_store = match PeerStore::new(10) {
            Ok(p) => Arc::new(p),
            Err(err) => return Err(err),
        };

        Ok(peer_store)
    }

    pub async fn start(&self) -> Result<(), String> {
        self.disc.start().await?;

        let disc_it = self.disc.iter();
        let a = disc_it.next().await?;
        println!("111,");

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
