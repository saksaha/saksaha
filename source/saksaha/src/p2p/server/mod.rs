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
use logger::{terr, tinfo};
use std::sync::Arc;
use tokio::sync::Semaphore;

const MAX_CONN_COUNT: usize = 50;

pub(crate) struct Server {
    pub(crate) disc_state: Arc<DiscState>,
    conn_semaphore: Arc<Semaphore>,
}

impl Server {
    pub fn new(disc_state: Arc<DiscState>) -> Server {
        let conn_semaphore = Arc::new(Semaphore::new(MAX_CONN_COUNT));

        Server {
            disc_state,
            conn_semaphore,
        }
    }

    pub fn start(&self) -> Result<(), String> {
        tinfo!(
            "p2p_discovery",
            "listener",
            "P2P discovery listener starts to accept requests",
        );

        self.run_loop()
    }

    pub fn run_loop(&self) -> Result<(), String> {
        let disc_state = self.disc_state.clone();
        let udp_conn = self.disc_state.udp_conn.clone();
        let conn_semaphore = self.conn_semaphore.clone();

        tokio::spawn(async move {
            loop {
                let conn_semaphore = conn_semaphore.clone();
                match conn_semaphore.acquire().await {
                    Ok(s) => s.forget(),
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "listener",
                            "Connection semaphore has been closed, err: {}",
                            err,
                        );
                        break;
                    }
                };

                let (msg, socket_addr) = match udp_conn.read_msg().await {
                    Some(m) => m,
                    None => {
                        continue;
                    }
                };

                let handler = Handler {
                    conn_semaphore,
                    disc_state: disc_state.clone(),
                    socket_addr,
                    msg,
                };

                match handler.run().await {
                    Ok(_) => (),
                    Err(err) => {
                        terr!(
                            "p2p_discovery",
                            "",
                            "Error processing request, addr: {}, err: {}",
                            socket_addr,
                            err
                        );
                    }
                };
            }
        });

        Ok(())
    }
}
