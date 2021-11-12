use log::{debug, info, warn};
use p2p_identity::Identity;
use p2p_transport::Connection;
use std::{net::ToSocketAddrs, sync::Arc};
use tokio::net::TcpListener;
use super::state::HostState;

pub(crate) struct Listener {
    tcp_listener: Arc<TcpListener>,
    host_state: Arc<HostState>,
}

impl Listener {
    pub fn new(
        tcp_listener: Arc<TcpListener>,
        host_state: Arc<HostState>,
    ) -> Listener {
        Listener {
            tcp_listener,
            host_state,
        }
    }

    pub fn start(&self) {
        self.run_loop();
    }

    pub fn run_loop(&self) {
        // let disc_state = self.disc_state.clone();
        // let udp_socket = self.udp_socket.clone();
        // let whoareyou_op = self.whoareyou_op.clone();
        let tcp_listener = self.tcp_listener.clone();

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

                debug!("Accepted new connection, endpoint: {}", addr);

                let mut handler = Handler {
                    conn: Connection::new(stream),
                };

                tokio::spawn(async move {
                    let _ = handler.run().await;
                });

                //     let mut buf = [0; 512];
                //     let (_, socket_addr) =
                //         match udp_socket.recv_from(&mut buf).await {
                //             Ok(res) => {
                //                 debug!(
                //                     "Accepted incoming request, len: {}, addr: {}",
                //                     res.0, res.1,
                //                 );
                //                 res
                //             }
                //             Err(err) => {
                //                 warn!("Error accepting request, err: {}", err);
                //                 continue;
                //             }
                //         };

                //     match Handler::run(
                //         disc_state.clone(),
                //         whoareyou_op.clone(),
                //         socket_addr,
                //         &buf,
                //     )
                //     .await
                //     {
                //         Ok(_) => (),
                //         Err(err) => {
                //             error!(
                //                 "Error processing request, addr: {}, err: {}",
                //                 socket_addr, err
                //             );
                //         }
                //     };
            }
        });
    }
}

struct Handler {
    conn: Connection,
}

impl Handler {
    async fn run(&mut self) -> Result<(), String> {
        let maybe_frame = match self.conn.read_frame().await {
            Ok(f) => f,
            Err(err) => return Err(format!("Can't read frame, err: {}", err)),
        };

        let frame = match maybe_frame {
            Some(f) => f,
            None => return Ok(()),
        };

        println!("frame: {}", frame);

        Ok(())
    }
}
