use super::handler::Handler;
use crate::{v1::net::Connection, AddrTable};
use futures::StreamExt;
use logger::{terr, tinfo, twarn};
use p2p_identity::Identity;
use std::{sync::Arc, time::Duration};
use tokio::sync::Semaphore;

const MAX_CONN_COUNT: usize = 50;

pub(crate) struct Server {
    udp_conn: Arc<Connection>,
    conn_semaphore: Arc<Semaphore>,
    identity: Arc<Identity>,
    addr_table: Arc<AddrTable>,
    addr_expire_duration: Duration,
}

pub(crate) struct ServerArgs {
    pub(crate) udp_conn: Arc<Connection>,
    pub(crate) identity: Arc<Identity>,
    pub(crate) addr_table: Arc<AddrTable>,
    pub(crate) addr_expire_duration: u64,
}

impl Server {
    pub fn new(server_args: ServerArgs) -> Server {
        let conn_semaphore = Arc::new(Semaphore::new(MAX_CONN_COUNT));

        let addr_expire_duration =
            Duration::from_secs(server_args.addr_expire_duration);

        Server {
            identity: server_args.identity,
            udp_conn: server_args.udp_conn,
            conn_semaphore,
            addr_table: server_args.addr_table,
            addr_expire_duration,
        }
    }

    pub async fn run(&self) {
        tinfo!(
            "p2p_discovery",
            "server",
            "P2P discovery server starts to accept requests",
        );

        self.run_loop().await;
    }

    pub async fn run_loop(&self) {
        let mut rx_lock = self.udp_conn.rx.write().await;

        loop {
            self.conn_semaphore.acquire().await.unwrap().forget();

            match rx_lock.next().await {
                Some(res) => {
                    match res {
                        Ok((msg, socket_addr)) => {
                            let handler = Handler {
                                conn_semaphore: self.conn_semaphore.clone(),
                            };

                            let udp_conn = self.udp_conn.clone();
                            let identity = self.identity.clone();
                            let table = self.addr_table.clone();
                            let addr_expire_duration =
                                self.addr_expire_duration;

                            tokio::spawn(async move {
                                match handler
                                    .run(
                                        msg,
                                        socket_addr,
                                        udp_conn,
                                        identity,
                                        table,
                                        addr_expire_duration,
                                    )
                                    .await
                                {
                                    Ok(_) => (),
                                    Err(err) => {
                                        terr!(
                                        "p2p_discovery",
                                        "server",
                                        "Error processing request, addr: {}, \
                                        err: {}",
                                        socket_addr,
                                        err
                                    );
                                    }
                                };
                            });
                        }
                        Err(err) => {
                            twarn!(
                                "p2p_discovery",
                                "server",
                                "Error parsing message, err: {}",
                                err
                            );
                        }
                    };
                }
                None => (),
            }
        }
    }
}
