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

        // let local_addr = format!("127.0.0.1:0");

        // let (listener, port) = match TcpListener::bind(local_addr).await {
        //     Ok(l) => {
        //         let local_addr = match l.local_addr() {
        //             Ok(addr) => addr,
        //             Err(err) => {
        //                 return Err(err.into());
        //             }
        //         };

        //         log!(DEBUG, "Start peer op listening, addr: {}\n", local_addr);

        //         (l, local_addr.port())
        //     }
        //     Err(err) => {
        //         return Err(err.into());
        //     }
        // };

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
