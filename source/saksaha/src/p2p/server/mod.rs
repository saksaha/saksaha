// use super::state::HostState;
// use logger::{tdebug, tinfo, twarn};
// use peer::{PeerValue, RegisteredPeerValue};
// use std::sync::Arc;
// use thiserror::Error;
// use tokio::net::{TcpListener, TcpStream};

// pub(crate) struct Server {
//     pub tcp_socket: Arc<TcpListener>,
//     host_state: Arc<HostState>,
// }

// #[derive(Error, Debug)]
// pub enum RequestHandleError {
//     #[error("Can't read stream")]
//     CannotReadStream {
//         #[from]
//         source: std::io::Error,
//     },

//     #[error("Invalid request")]
//     Invalid,

//     #[error("No available peer slot, err: {err}")]
//     NoAvailablePeerSlot { err: String },
// }

// impl Server {
//     pub fn new(
//         tcp_socket: Arc<TcpListener>,
//         host_state: Arc<HostState>,
//     ) -> Server {
//         Server {
//             tcp_socket,
//             host_state,
//         }
//     }

//     pub fn start(&self) {
//         tinfo!("p2p", "", "Starting accepting requests");

//         self.run_loop();
//     }

//     pub fn run_loop(&self) {
//         let tcp_socket = self.tcp_socket.clone();
//         let host_state = self.host_state.clone();

//         tokio::spawn(async move {
//             loop {
//                 let (stream, addr) = match tcp_socket.accept().await {
//                     Ok(s) => s,
//                     Err(err) => {
//                         twarn!(
//                             "saksaha",
//                             "p2p",
//                             "Error accepting connection request, err: {}",
//                             err,
//                         );

//                         continue;
//                     }
//                 };

//                 tdebug!("p2p", "Accepted new connection, endpoint: {}", addr);

//                 let mut handler = Handler {};
//                 let host_state = host_state.clone();

//                 // tokio::spawn(async move {
//                 //     let _ = handler.run(stream, host_state).await;
//                 // });
//             }
//         });
//     }
// }

// struct Handler {}

// impl Handler {
//     async fn run(
//         &mut self,
//         stream: TcpStream,
//         host_state: Arc<HostState>,
//     ) -> Result<(), RequestHandleError> {
//         let peer = match host_state.peer_store.reserve().await {
//             Ok(p) => p,
//             Err(err) => {
//                 return Err(RequestHandleError::NoAvailablePeerSlot { err })
//             }
//         };

//         // match p2p_transport::receive_handshake(
//         //     stream,
//         //     host_state.identity.clone(),
//         // )
//         // .await
//         // {
//         //     Ok(t) => {
//         //         let mut p_val = peer.value.lock().await;
//         //         *p_val =
//         //             PeerValue::Registered(RegisteredPeerValue { transport: t });
//         //         std::mem::drop(p_val);

//         //         host_state.peer_store.register(peer.clone()).await;
//         //     }
//         //     Err(err) => return Err(RequestHandleError::Invalid),
//         // };

//         Ok(())
//     }
// }

mod handler;

use super::state::HostState;
use handler::Handler;
use logger::{tdebug, terr, tinfo, twarn};
use p2p_transport::connection::Connection;
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Semaphore,
};

const MAX_CONN_COUNT: usize = 50;

pub(crate) struct Server {
    host_state: Arc<HostState>,
    conn_semaphore: Arc<Semaphore>,
    p2p_socket: TcpListener,
}

impl Server {
    pub fn new(
        host_state: Arc<HostState>,
        p2p_max_conn_count: Option<u16>,
        p2p_socket: TcpListener,
    ) -> Server {
        let p2p_max_conn_count = match p2p_max_conn_count {
            Some(c) => c.into(),
            None => MAX_CONN_COUNT,
        };

        let conn_semaphore = Arc::new(Semaphore::new(p2p_max_conn_count));

        Server {
            host_state,
            conn_semaphore,
            p2p_socket,
        }
    }

    pub async fn run(&self) {
        tinfo!("saksaha", "p2p", "P2P server starts to accept requests",);

        self.run_loop().await;
    }

    async fn accept(&self) -> Result<TcpStream, String> {
        let mut backoff = 1;

        loop {
            match self.p2p_socket.accept().await {
                Ok((socket, _)) => return Ok(socket),
                Err(err) => {
                    if backoff > 64 {
                        // Accept has failed too many times. Return the error.
                        return Err(err.to_string());
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(backoff)).await;

            // Double the back off
            backoff *= 2;
        }
    }

    pub async fn run_loop(&self) {
        let conn_semaphore = self.conn_semaphore.clone();

        loop {
            let conn_semaphore = conn_semaphore.clone();
            conn_semaphore.acquire().await.unwrap().forget();

            let socket = match self.accept().await {
                Ok(s) => s,
                Err(err) => {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error accepting tcp request, err: {}",
                        err
                    );

                    continue;
                }
            };

            let mut conn = match Connection::new(socket) {
                Ok((c, peer_addr)) => {
                    tdebug!(
                        "saksaha",
                        "p2p",
                        "(callee) Accepted a tcp connection from source, \
                        peer_addr: {:?}",
                        peer_addr,
                    );

                    c
                }
                Err(err) => {
                    tdebug!(
                        "saksaha",
                        "p2p",
                        "(callee) Cannot create a connection, err: {}",
                        err,
                    );

                    continue;
                }
            };

            let mut handler = Handler {
                conn_semaphore: conn_semaphore.clone(),
                host_state: self.host_state.clone(),
            };

            tokio::spawn(async move {
                if let Err(err) = handler.run(conn).await {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error handling p2p request, err: {}",
                        err
                    );
                }
            });
        }
    }
}
