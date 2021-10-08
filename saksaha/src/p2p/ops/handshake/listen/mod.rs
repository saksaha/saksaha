use std::sync::Arc;

use crate::{
    common::{Error, Result},
    err, msg_err,
    node::task_manager::TaskManager,
    p2p::credential::Credential,
};
use logger::log;
use tokio::{net::TcpListener, sync::mpsc::Sender as MpscSender};

pub struct Listen {}

impl Listen {
    pub fn new() -> Listen {
        let listen = Listen {};

        listen
    }

    pub async fn start(
        &self,
        disc_wakeup_tx: Arc<MpscSender<usize>>,
        task_mng: Arc<TaskManager>,
        credential: Arc<Credential>,
        peer_op_listener: TcpListener,
    ) -> Result<()> {
        log!(DEBUG, "Start listen - handshake\n");

        let dial_loop_tx = disc_wakeup_tx.clone();
        tokio::spawn(async move {
            let tx = dial_loop_tx;

            loop {
                let (mut stream, addr) = match peer_op_listener.accept().await {
                    Ok(res) => res,
                    Err(err) => {
                        return;
                    }
                };

                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    loop {
                        // let n = match
                    }
                });

                println!("22222");
            }
        });

        Ok(())
    }
}
