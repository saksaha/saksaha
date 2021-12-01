use log::{debug, info, warn};
use p2p_identity::Identity;
use p2p_transport::{Connection, Frame};
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

            }
        });
    }
}

struct Handler {
    conn: Connection,
}

impl Handler {
    async fn run(&mut self) -> Result<(), String> {

        println!("22");
        let maybe_frame = match self.conn.read_frame().await {
            Ok(f) => f,
            Err(err) => return Err(format!("Can't read frame, err: {}", err)),
        };

        println!("11");

        let frame = match maybe_frame {
            Some(fr) => {
                match fr {
                    Frame::Array(ref fr) => {
                        let a = fr.as_slice();

                        for e in a {
                            let b = e.to_string();
                            println!("1, b: {}", e);
                        }

                        println!("3, {:?}", a);
                    },
                    _ => ()
                }
            },
            None => return Ok(()),
        };

        // println!("handler frame: {}", frame);

        Ok(())
    }
}
