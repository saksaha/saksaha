use std::sync::Arc;

use crate::{common::{Error, Result}, err_res, msg_err, node::task_manager::{MsgKind, TaskManager}};
use logger::log;
use tokio::{
    net::TcpListener, sync::mpsc::Sender as MpscSender,
};

pub struct Listen {
    dial_loop_tx: Arc<MpscSender<usize>>,
    task_mng: Arc<TaskManager>,
    listener: Arc<TcpListener>,
    port: u16,
}

impl Listen {
    pub async fn new(
        dial_loop_tx: Arc<MpscSender<usize>>,
        task_mng: Arc<TaskManager>,
    ) -> Result<Listen> {
        let local_addr = format!("127.0.0.1:0");

        let (listener, port) = match TcpListener::bind(local_addr).await {
            Ok(l) => {
                let local_addr = match l.local_addr() {
                    Ok(addr) => addr,
                    Err(err) => {
                        let msg = msg_err!(
                            MsgKind::SetupFailure,
                            "Error getting peer op local addr, err: {}",
                            err
                        );

                        task_mng.send(msg.to_owned()).await;
                        return Err(Error::from(msg));
                    }
                };

                log!(DEBUG, "Start peer op listening, addr: {}\n", local_addr);

                (l, local_addr.port())
            },
            Err(err) => {
                let msg = msg_err!(
                    MsgKind::SetupFailure,
                    "Error binding peer op endpoint, err: {}",
                    err
                );

                task_mng.send(msg).await;
                return err_res!("a");
            }
        };

        let listen = Listen {
            dial_loop_tx,
            task_mng,
            listener: Arc::new(listener),
            port,
        };

        Ok(listen)
    }

    pub async fn start_listening(&self) {
        let listener = self.listener.clone();

        tokio::spawn(async move {
            loop {
                let (mut stream, addr) = match listener.accept().await {
                    Ok(res) => res,
                    Err(err) => {
                        return;
                        // return err_res!("Error accepting a request, err: {}", err);
                    }
                };

                tokio::spawn(async move {
                    let mut buf = [0; 1024];

                    loop {
                        // let n = match
                    }
                });
            }
        });

        // Ok(listener)
    }
}
