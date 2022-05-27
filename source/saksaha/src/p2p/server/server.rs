use logger::{tdebug, terr, tinfo, twarn};
use p2p_identity::Identity;
use p2p_peer::PeerTable;
use p2p_transport::Connection;
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Semaphore,
};

use super::handler::Handler;

const MAX_CONN_COUNT: usize = 50;

pub(crate) struct Server {
    conn_semaphore: Arc<Semaphore>,
    p2p_socket: TcpListener,
    identity: Arc<Identity>,
    peer_table: Arc<PeerTable>,
}

impl Server {
    pub fn new(
        p2p_max_conn_count: Option<u16>,
        p2p_socket: TcpListener,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
    ) -> Server {
        let p2p_max_conn_count = match p2p_max_conn_count {
            Some(c) => c.into(),
            None => MAX_CONN_COUNT,
        };

        let conn_semaphore = Arc::new(Semaphore::new(p2p_max_conn_count));

        Server {
            conn_semaphore,
            p2p_socket,
            identity,
            peer_table,
        }
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

    pub async fn run(&self) {
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
                Ok(c) => {
                    tdebug!(
                        "saksaha",
                        "p2p",
                        "Accepted a tcp connection from source, \
                        peer_addr: {:?}",
                        c.socket_addr,
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
            };

            let identity = self.identity.clone();
            let peer_table = self.peer_table.clone();

            tokio::spawn(async move {
                if let Err(err) = handler.run(conn, identity, peer_table).await
                {
                    twarn!(
                        "saksaha",
                        "p2p",
                        "Error handling p2p request, err: {}",
                        err,
                    );
                }
            });
        }
    }
}
