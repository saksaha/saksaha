use super::handler::Handler;
use crate::p2p::P2PHostError;
use colored::Colorize;
use sak_logger::{debug, info, warn};
use sak_p2p_discovery::AddrTable;
use sak_p2p_id::Identity;
use sak_p2p_peertable::PeerTable;
use sak_p2p_transport::Conn;
use std::{sync::Arc, time::Duration};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::Semaphore,
};

const MAX_CONN_COUNT: usize = 50;

pub(crate) struct Server {
    conn_semaphore: Arc<Semaphore>,
    p2p_socket: TcpListener,
    identity: Arc<Identity>,
    peer_table: Arc<PeerTable>,
    addr_table: Arc<AddrTable>,
}

impl Server {
    pub fn new(
        p2p_max_conn_count: Option<u16>,
        p2p_socket: TcpListener,
        identity: Arc<Identity>,
        peer_table: Arc<PeerTable>,
        addr_table: Arc<AddrTable>,
    ) -> Result<Server, P2PHostError> {
        let p2p_max_conn_count = match p2p_max_conn_count {
            Some(c) => c.into(),
            None => MAX_CONN_COUNT,
        };

        let conn_semaphore = Arc::new(Semaphore::new(p2p_max_conn_count));

        let local_addr = p2p_socket.local_addr()?;

        info!(
            "Bound tcp socket for P2P host, addr: {}",
            local_addr.to_string().yellow(),
        );

        let s = Server {
            conn_semaphore,
            p2p_socket,
            identity,
            peer_table,
            addr_table,
        };

        Ok(s)
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
                    warn!("Error accepting tcp request, err: {}", err);

                    continue;
                }
            };

            let conn = match Conn::new(
                socket,
                self.identity.credential.public_key_str.to_string(),
            ) {
                Ok(c) => {
                    debug!(
                        "Accepted a tcp connection from source, \
                        peer_addr: {:?}",
                        c.socket_addr,
                    );

                    c
                }
                Err(err) => {
                    debug!("(callee) Cannot create a connection, err: {}", err,);

                    continue;
                }
            };

            let mut handler = Handler {
                conn_semaphore: conn_semaphore.clone(),
            };

            let identity = self.identity.clone();
            let peer_table = self.peer_table.clone();
            let addr_table = self.addr_table.clone();

            tokio::spawn(async move {
                handler.run(conn, identity, peer_table, addr_table).await;
            });
        }
    }
}
