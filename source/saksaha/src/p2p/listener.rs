use super::state::HostState;
use logger::{tdebug, tinfo, twarn};
use peer::{PeerValue, RegisteredPeerValue};
use std::sync::Arc;
use thiserror::Error;
use tokio::net::{TcpListener, TcpStream};

pub(crate) struct Listener {
    pub tcp_socket: Arc<TcpListener>,
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
        tcp_socket: Arc<TcpListener>,
        host_state: Arc<HostState>,
    ) -> Listener {
        Listener {
            tcp_socket,
            host_state,
        }
    }

    pub fn start(&self) {
        tinfo!("p2p", "Starting accepting requests");

        self.run_loop();
    }

    pub fn run_loop(&self) {
        let tcp_socket = self.tcp_socket.clone();
        let host_state = self.host_state.clone();

        tokio::spawn(async move {
            loop {
                let (stream, addr) = match tcp_socket.accept().await {
                    Ok(s) => s,
                    Err(err) => {
                        twarn!(
                            "p2p",
                            "Error accepting connection request, err: {}",
                            err,
                        );

                        continue;
                    }
                };

                tdebug!("p2p", "Accepted new connection, endpoint: {}", addr);

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
