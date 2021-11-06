use crate::{p2p::credential::Credential, peer::peer_store::PeerStore};
use log::{debug, info};
use saksaha_p2p_identity::Identity;
use std::sync::Arc;
use tokio::{
    net::TcpListener,
    sync::{mpsc::Receiver, Mutex},
};

pub struct Listener {
    credential: Arc<Box<dyn Identity + Send + Sync>>,
    peer_store: Arc<PeerStore>,
    tcp_listener: Mutex<Option<TcpListener>>,
    rpc_port: u16,
    p2p_port: Mutex<u16>,
}

impl Listener {
    pub fn new(
        credential: Arc<Box<dyn Identity + Send + Sync>>,
        peer_store: Arc<PeerStore>,
        // tcp_listener: TcpListener,
        rpc_port: u16,
        p2p_port: Option<u16>,
    ) -> Listener {
        let p2p_port = match p2p_port {
            Some(p) => p,
            None => 0,
        };

        Listener {
            credential,
            peer_store,
            tcp_listener: Mutex::new(None),
            rpc_port,
            p2p_port: Mutex::new(p2p_port),
        }
    }

    pub async fn get_port(&self) -> Result<u16, String> {
        let tcp_listener = self.tcp_listener.lock().await;

        if let None = *tcp_listener {
            self.start_server().await
        } else {
            let port = self.p2p_port.lock().await.clone();
            Ok(port)
        }
    }

    pub async fn start_server(&self) -> Result<u16, String> {
        let p2p_port = self.p2p_port.lock().await;

        let (tcp_listener, tcp_port) = {
            let local_addr = format!("127.0.0.1:{}", p2p_port);

            match TcpListener::bind(local_addr).await {
                Ok(listener) => match listener.local_addr() {
                    Ok(local_addr) => {
                        info!(
                            "P2P tcp listener is bound, addr: {}",
                            &local_addr
                        );

                        (listener, local_addr.port())
                    }
                    Err(err) => {
                        return Err(format!(
                            "Can't get local address of p2p listener, err: {}",
                            err
                        ))
                    }
                },
                Err(err) => {
                    return Err(format!(
                        "Can't bind tcp listener, err: {}",
                        err
                    ))
                }
            }
        };

        let mut tcp_listener_lock = self.tcp_listener.lock().await;
        *tcp_listener_lock = Some(tcp_listener);

        Ok(tcp_port)
    }

    pub async fn start(
        &self,
    ) -> Result<(), String> {
        self.start_server().await;

        Ok(())
    }
}
