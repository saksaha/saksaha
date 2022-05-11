mod handler;

use super::DiscState;
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

    pub async fn run(&self) {
        tinfo!(
            "p2p_discovery",
            "server",
            "P2P discovery server starts to accept requests",
        );

        self.run_loop().await;
    }

    pub async fn run_loop(&self) {
        loop {
            self.conn_semaphore.acquire().await.unwrap().forget();

            let (msg, socket_addr) =
                match self.disc_state.udp_conn.read_msg().await {
                    Some(m) => m,
                    None => {
                        continue;
                    }
                };

            let handler = Handler {
                conn_semaphore: self.conn_semaphore.clone(),
                disc_state: self.disc_state.clone(),
                socket_addr,
                msg,
            };

            match handler.run().await {
                Ok(_) => (),
                Err(err) => {
                    terr!(
                        "p2p_discovery",
                        "server",
                        "Error processing request, addr: {}, err: {}",
                        socket_addr,
                        err
                    );
                }
            };
        }
    }
}
