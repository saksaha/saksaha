use std::sync::Arc;

use crate::{
    common::{Error, Result},
    err, msg_err,
    node::task_manager::{TaskManager},
};
use logger::log;
use tokio::{net::TcpListener, sync::mpsc::Sender as MpscSender};

pub struct Listen {
    dial_loop_tx: Arc<MpscSender<usize>>,
    task_mng: Arc<TaskManager>,
}

impl Listen {
    pub fn new(
        dial_loop_tx: Arc<MpscSender<usize>>,
        task_mng: Arc<TaskManager>,
    ) -> Listen {
        let listen = Listen {
            dial_loop_tx,
            task_mng,
        };

        listen
    }

    pub async fn start(&self) -> Result<u16> {
        let local_addr = format!("127.0.0.1:0");

        let (listener, port) = match TcpListener::bind(local_addr).await {
            Ok(l) => {
                let local_addr = match l.local_addr() {
                    Ok(addr) => addr,
                    Err(err) => {
                        return Err(err.into());
                    }
                };

                log!(DEBUG, "Start peer op listening, addr: {}\n", local_addr);

                (l, local_addr.port())
            }
            Err(err) => {
                return Err(err.into());
            }
        };

        let dial_loop_tx = self.dial_loop_tx.clone();
        tokio::spawn(async move {
            let tx = dial_loop_tx;

            loop {
                let (mut stream, addr) = match listener.accept().await {
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

        Ok(port)
    }
}
