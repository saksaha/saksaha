use crate::msg::{Msg, MsgType, WhoAreYouSyn};

use super::{
    instr::whoareyou::{self, initiate, receive},
    // ops::Opcode,
    DiscState,
};
use logger::{tdebug, terr, tinfo, twarn};
use std::{net::SocketAddr, sync::Arc};
use thiserror::Error;
use tokio::{net::UdpSocket, sync::Semaphore};

const MAX_CONN_COUNT: usize = 50;

#[derive(Error, Debug)]
pub enum ListenerError {
    #[error("Already has active call with endpoint, {0}")]
    CallAlreadyInProgress(String),
}

pub(crate) struct Listener {
    pub(crate) disc_state: Arc<DiscState>,
    conn_semaphore: Arc<Semaphore>,
}

impl Listener {
    pub fn new(disc_state: Arc<DiscState>) -> Listener {
        let conn_semaphore = Arc::new(Semaphore::new(MAX_CONN_COUNT));

        Listener {
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

struct Handler {
    conn_semaphore: Arc<Semaphore>,
    disc_state: Arc<DiscState>,
    socket_addr: SocketAddr,
    msg: Msg,
}

impl Handler {
    async fn run(&self) -> Result<(), String> {
        match self.msg.msg_type {
            MsgType::WhoAreYouSyn => {
                let way_syn = match WhoAreYouSyn::from_msg(&self.msg) {
                    Ok(w) => w,
                    Err(err) => {
                        return Err(format!(
                            "Error parsing who are you syn msg, err: {}",
                            err
                        ));
                    }
                };

                whoareyou::recv_who_are_you(self.disc_state.clone(), way_syn);
            }
        };

        Ok(())
    }
}

impl Drop for Handler {
    fn drop(&mut self) {
        self.conn_semaphore.add_permits(1);
    }
}
