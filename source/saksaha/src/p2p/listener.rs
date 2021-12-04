use super::state::HostState;
use log::{debug, info, warn};
use p2p_identity::Identity;
use p2p_transport::{Connection, Frame};
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

                let mut handler = Handler { stream };

                tokio::spawn(async move {
                    let _ = handler.run().await;
                });
            }
        });
    }
}

struct Handler {
    stream: TcpStream,
}

impl Handler {
    async fn run(&mut self) -> Result<(), RequestHandleError> {
        let mut buf = vec![0; 512];
        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                break;
            }
        }

        println!("buf: {:?}", buf);

        // let frame = match maybe_frame {
        //     Some(fr) => {
        //         match fr {
        //             Frame::Array(ref fr) => {
        //                 let a = fr.as_slice();

        //                 for e in a {
        //                     let b = e.to_string();
        //                     println!("1, b: {}", e);
        //                 }

        //                 println!("3, {:?}", a);
        //             },
        //             _ => ()
        //         }
        //     },
        //     None => return Ok(()),
        // };

        // println!("handler frame: {}", frame);

        Ok(())
    }
}
