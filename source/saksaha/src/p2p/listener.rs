use super::state::HostState;
use bytes::BytesMut;
use log::{debug, info, warn};
use p2p_identity::Identity;
use p2p_transport::{Connection, Frame, HANDSHAKE_CODE};
use peer::{PeerValue, RegisteredPeerValue};
use std::{net::ToSocketAddrs, sync::Arc};
use thiserror::Error;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

pub(crate) struct Listener {
    tcp_listener: Arc<TcpListener>,
    host_state: Arc<HostState>,
}

#[derive(Error, Debug)]
pub enum RequestHandleError {
    #[error("Can't read stream")]
    CannotReadStream {
        #[from]
        source: std::io::Error,
    },

    #[error("Invalid request")]
    Invalid,

    #[error("No available peer slot, err: {err}")]
    NoAvailablePeerSlot { err: String },
}

impl Listener {
    pub fn new(
        tcp_listener: Arc<TcpListener>,
        host_state: Arc<HostState>,
    ) -> Listener {
        info!(
            "P2P Listener is initialized, local_addr: {:?}",
            tcp_listener.local_addr()
        );

        Listener {
            tcp_listener,
            host_state,
        }
    }

    pub fn start(&self) {
        self.run_loop();
    }

    pub fn run_loop(&self) {
        let tcp_listener = self.tcp_listener.clone();
        let host_state = self.host_state.clone();

        tokio::spawn(async move {
            loop {
                let (stream, addr) = match tcp_listener.accept().await {
                    Ok(s) => s,
                    Err(err) => {
                        warn!(
                            "Error accepting connection request, err: {}",
                            err,
                        );

                        continue;
                    }
                };

                debug!("[p2p] Accepted new connection, endpoint: {}", addr);

                let mut handler = Handler {};
                let host_state = host_state.clone();

                tokio::spawn(async move {
                    let _ = handler.run(stream, host_state).await;
                });
            }
        });
    }
}

struct Handler {}

impl Handler {
    async fn run(
        &mut self,
        stream: TcpStream,
        host_state: Arc<HostState>,
    ) -> Result<(), RequestHandleError> {
        let peer = match host_state.peer_store.reserve().await {
            Ok(p) => p,
            Err(err) => {
                return Err(RequestHandleError::NoAvailablePeerSlot { err })
            }
        };

        match p2p_transport::receive_handshake(
            stream,
            host_state.identity.clone(),
        )
        .await
        {
            Ok(t) => {
                let mut p_val = peer.value.lock().await;
                *p_val =
                    PeerValue::Registered(RegisteredPeerValue { transport: t });
                std::mem::drop(p_val);

                host_state.peer_store.register(peer.clone()).await;
            }
            Err(err) => return Err(RequestHandleError::Invalid),
        };

        Ok(())
    }
}
