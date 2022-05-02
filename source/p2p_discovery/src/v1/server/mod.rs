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
