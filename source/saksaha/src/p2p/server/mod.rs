mod handler;
mod request;

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

            let conn = match Connection::new(socket) {
                Ok((c, peer_addr)) => {
                    tdebug!(
                        "saksaha",
                        "p2p",
                        "Accepted a tcp connection from source, \
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
                if let Err(_err) = handler.run(conn).await {
                    // twarn!(
                    //     "saksaha",
                    //     "p2p",
                    //     "Error handling p2p request, err: {}",
                    //     err
                    // );
                }
            });
        }
    }
}
